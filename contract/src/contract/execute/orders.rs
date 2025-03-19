use std::u128;

use crate::contract::{query, DENOM};
use crate::msg::OrderSort;
use crate::state::{
    BuyOrder, Sale, SellOrder, Share, BUY_ORDERS, BUY_ORDER_COUNT, SALES, SALE_COUNT, SELL_ORDERS,
    SELL_ORDER_COUNT, SHARES, SHARE_COUNT, STOCKS,
};
use crate::ContractError;
use cosmwasm_std::{coins, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response};

use format as f;

pub fn cancel_buy_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    buy_order_id: u64,
) -> Result<Response, ContractError> {
    let mut buy_order =
        query::orders::get_buy_order_by_id(deps.as_ref(), env.clone(), buy_order_id)?.order;

    if info.sender.clone() != buy_order.owner {
        return Err(ContractError::Unauthorized);
    }

    if buy_order.resolved_at.is_some() {
        return Err(ContractError::GenericError(f!(
            "Order has already been resolved"
        )));
    }

    let current_timestamp = env.block.time.nanos() / 1_000_000; // to milliseconds

    buy_order.resolved_at = Some(current_timestamp);

    BUY_ORDERS.save(deps.storage, &buy_order_id.to_be_bytes(), &buy_order)?;

    Ok(Response::new().add_attribute("action", "cancel_buy_order"))
}

pub fn cancel_sell_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sell_order_id: u64,
) -> Result<Response, ContractError> {
    let mut sell_order =
        query::orders::get_sell_order_by_id(deps.as_ref(), env.clone(), sell_order_id)?.order;

    let current_timestamp = env.block.time.nanos() / 1_000_000; // to milliseconds

    if info.sender.clone() != sell_order.owner {
        return Err(ContractError::Unauthorized);
    }

    if sell_order.resolved_at.is_some() {
        return Err(ContractError::GenericError(f!(
            "Order has already been resolved"
        )));
    }

    sell_order.resolved_at = Some(current_timestamp);

    SELL_ORDERS.save(deps.storage, &sell_order_id.to_be_bytes(), &sell_order)?;

    Ok(Response::new().add_attribute("action", "cancel_sell_order"))
}

pub fn create_buy_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stock_id: u64,
    shares: u64,
    price_per_share: u128,
) -> Result<Response, ContractError> {
    // Validate inputs
    if shares == 0 {
        return Err(ContractError::GenericError(f!(
            "Cannot create order with 0 shares"
        )));
    }
    if price_per_share == 0 {
        return Err(ContractError::GenericError(f!(
            "Price per share must be greater than 0"
        )));
    }

    // Load the stock and check if it's in sale (auction ended)
    let stock = STOCKS
        .load(deps.storage, &stock_id.to_be_bytes())
        .map_err(|_| ContractError::NotFound(f!("Stock with id {stock_id}")))?;

    let current_timestamp = env.block.time.nanos() / 1_000_000;

    if stock.auction_end.is_none() || stock.auction_end > Some(current_timestamp) {
        return Err(ContractError::GenericError(f!("Stock is not in sale")));
    }

    // Ensure the buyer has sent enough funds
    let required_amount = shares as u128 * price_per_share;

    let sent_funds = info.funds.iter().find(|coin| coin.denom == DENOM);

    if sent_funds.is_none() {
        return Err(ContractError::GenericError(f!("No {DENOM} sent")));
    }

    let sent_amount = sent_funds.unwrap().amount.u128();

    if sent_amount < required_amount {
        return Err(ContractError::GenericError(f!(
            "Insufficient funds: sent {sent_amount}, required {required_amount}"
        )));
    }

    // Get sell orders to match with
    let open_sell_orders = query::orders::get_open_sell_orders_by_stock_id(
        deps.as_ref(),
        env.clone(),
        stock_id,
        OrderSort::PriceAsc,
    )?
    .orders;

    let mut remaining_shares = shares;
    let mut messages = vec![];
    let mut total_cost = 0;

    // match sell orders
    for mut sell_order in open_sell_orders {
        if sell_order.price_per_share > price_per_share {
            break;
        }

        let balance = sell_order.available_shares - sell_order.sold_shares;

        let take = std::cmp::min(remaining_shares, balance);

        sell_order.sold_shares += take;

        if sell_order.sold_shares == sell_order.available_shares {
            sell_order.resolved_at = Some(current_timestamp);
        }

        SELL_ORDERS.save(deps.storage, &sell_order.id.to_be_bytes(), &sell_order)?;

        let cost = take as u128 * sell_order.price_per_share;
        total_cost += cost;

        // Transfer share from seller to buyer
        let mut seller_share = query::shares::get_shares_by_owner(
            deps.as_ref(),
            env.clone(),
            sell_order.owner.clone(),
        )?
        .shares
        .into_iter()
        .filter(|share| share.stock_id == stock_id)
        .nth(0)
        .ok_or(ContractError::GenericError("This shouldn't happen".into()))?;

        seller_share.no_of_shares -= take;
        SHARES.save(deps.storage, &seller_share.id.to_be_bytes(), &seller_share)?;

        let buyer_share =
            query::shares::get_shares_by_owner(deps.as_ref(), env.clone(), info.sender.clone())?
                .shares
                .into_iter()
                .filter(|share| share.stock_id == stock_id)
                .nth(0);

        // if buyer already have shares in stock update share count
        if let Some(mut buyer_share) = buyer_share {
            buyer_share.no_of_shares += take;
            SHARES.save(deps.storage, &buyer_share.id.to_be_bytes(), &buyer_share)?;
        } else {
            // else create new share for buyer
            let new_share_id = SHARE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
            SHARE_COUNT.save(deps.storage, &new_share_id)?;

            let new_share = Share {
                id: new_share_id,
                stock_id,
                no_of_shares: take,
                owner: info.sender.clone(),
            };

            SHARES.save(deps.storage, &new_share_id.to_be_bytes(), &new_share)?;
        }

        // Create Sale record
        let sale_id = SALE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
        SALE_COUNT.save(deps.storage, &sale_id)?;

        let sale = Sale {
            id: sale_id,
            stock_id,
            no_of_shares: take,
            price_per_share: sell_order.price_per_share,
            from: sell_order.owner.clone(),
            to: info.sender.clone(),
            created_at: current_timestamp,
        };

        SALES.save(deps.storage, &sale_id.to_be_bytes(), &sale)?;

        // Send funds to seller
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: sell_order.owner.to_string(),
            amount: coins(cost, DENOM),
        }));

        remaining_shares -= take;
        if remaining_shares == 0 {
            break;
        }
    }

    // Refund excess funds
    let price_of_remaining_shares = remaining_shares as u128 * price_per_share;

    let excess_funds = sent_amount - (total_cost + price_of_remaining_shares);

    if excess_funds > 0 {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(excess_funds, DENOM),
        }));
    }

    let mut response = Response::new()
        .add_attribute("action", "create_buy_order")
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute("price_per_share", price_per_share.to_string())
        .add_messages(messages);

    // Create BuyOrder
    let resolved_at = if remaining_shares > 0 {
        None
    } else {
        // close order
        Some(current_timestamp)
    };

    let buy_order_id = BUY_ORDER_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
    BUY_ORDER_COUNT.save(deps.storage, &buy_order_id)?;

    let buy_order = BuyOrder {
        id: buy_order_id,
        stock_id,
        requested_shares: shares,
        price_per_share,
        bought_shares: shares - remaining_shares,
        owner: info.sender.clone(),
        created_at: current_timestamp,
        resolved_at,
    };

    BUY_ORDERS.save(deps.storage, &buy_order_id.to_be_bytes(), &buy_order)?;
    response = response
        .add_attribute("buy_order_id", buy_order_id.to_string())
        .add_attribute("remaining_shares", remaining_shares.to_string());

    Ok(response)
}

pub fn create_sell_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stock_id: u64,
    shares: u64,
    price_per_share: u128,
) -> Result<Response, ContractError> {
    // Validate inputs
    if shares == 0 {
        return Err(ContractError::GenericError(f!(
            "Cannot create order with 0 shares"
        )));
    }
    if price_per_share == 0 {
        return Err(ContractError::GenericError(f!(
            "Price per share must be greater than 0"
        )));
    }

    // Load the stock and check if it's in sale (auction ended)
    let stock = STOCKS
        .load(deps.storage, &stock_id.to_be_bytes())
        .map_err(|_| ContractError::NotFound(f!("Stock with id {stock_id}")))?;

    let current_timestamp = env.block.time.nanos() / 1_000_000;

    if stock.auction_end.is_none() || stock.auction_end > Some(current_timestamp) {
        return Err(ContractError::GenericError(f!("Stock is not in sale")));
    }

    // Check seller has enough shares in this stock
    let mut seller_share =
        query::shares::get_shares_by_owner(deps.as_ref(), env.clone(), info.sender.clone())?
            .shares
            .into_iter()
            .filter(|share| share.stock_id == stock_id)
            .nth(0)
            .ok_or(ContractError::GenericError(
                "You do not have shares in this stock".into(),
            ))?;

    if seller_share.no_of_shares < shares {
        return Err(ContractError::GenericError(f!(
            "Insufficient shares. Has {}, needs {shares}",
            seller_share.no_of_shares
        )));
    }

    let mut remaining_shares = shares;
    let mut messages = vec![];

    // Get buy orders to match with
    let open_buy_orders = query::orders::get_open_buy_orders_by_stock_id(
        deps.as_ref(),
        env.clone(),
        stock_id,
        OrderSort::PriceDesc,
    )?
    .orders;

    // match buy orders
    for mut buy_order in open_buy_orders {
        if buy_order.price_per_share < price_per_share {
            break;
        }

        let balance = buy_order.requested_shares - buy_order.bought_shares;

        let take = std::cmp::min(remaining_shares, balance);

        buy_order.bought_shares += take;

        if buy_order.bought_shares == buy_order.requested_shares {
            buy_order.resolved_at = Some(current_timestamp);
        }

        BUY_ORDERS.save(deps.storage, &buy_order.id.to_be_bytes(), &buy_order)?;

        // Transfer shares from seller to buyer
        seller_share.no_of_shares -= take;
        SHARES.save(deps.storage, &seller_share.id.to_be_bytes(), &seller_share)?;

        let buyer_share = query::shares::get_shares_by_owner(
            deps.as_ref(),
            env.clone(),
            buy_order.owner.clone(),
        )?
        .shares
        .into_iter()
        .filter(|share| share.stock_id == stock_id)
        .nth(0);

        // if buyer already have shares in stock update share count
        if let Some(mut buyer_share) = buyer_share {
            buyer_share.no_of_shares += take;
            SHARES.save(deps.storage, &buyer_share.id.to_be_bytes(), &buyer_share)?;
        } else {
            // else create new share for buyer
            let new_share_id = SHARE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
            SHARE_COUNT.save(deps.storage, &new_share_id)?;

            let new_share = Share {
                id: new_share_id,
                stock_id,
                no_of_shares: take,
                owner: buy_order.owner.clone(),
            };

            SHARES.save(deps.storage, &new_share_id.to_be_bytes(), &new_share)?;
        }

        // Create Sale record
        let sale_id = SALE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
        SALE_COUNT.save(deps.storage, &sale_id)?;

        let sale = Sale {
            id: sale_id,
            stock_id,
            no_of_shares: take,
            price_per_share: buy_order.price_per_share,
            from: info.sender.clone(),
            to: buy_order.owner.clone(),
            created_at: current_timestamp,
        };

        SALES.save(deps.storage, &sale_id.to_be_bytes(), &sale)?;

        let cost = take as u128 * buy_order.price_per_share;

        // Send funds to seller
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(cost, DENOM),
        }));

        remaining_shares -= take;
        if remaining_shares == 0 {
            break;
        }
    }

    // Create SellOrder
    let resolved_at = if remaining_shares > 0 {
        None
    } else {
        Some(current_timestamp)
    };

    let sell_order_id = SELL_ORDER_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
    SELL_ORDER_COUNT.save(deps.storage, &sell_order_id)?;

    let sell_order = SellOrder {
        id: sell_order_id,
        stock_id,
        available_shares: shares,
        price_per_share,
        sold_shares: shares - remaining_shares,
        owner: info.sender.clone(),
        created_at: current_timestamp,
        resolved_at,
    };

    SELL_ORDERS.save(deps.storage, &sell_order_id.to_be_bytes(), &sell_order)?;

    let response = Response::new()
        .add_attribute("action", "create_sell_order")
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute("price_per_share", price_per_share.to_string())
        .add_attribute("sell_order_id", sell_order_id.to_string())
        .add_attribute("remaining_shares", remaining_shares.to_string())
        .add_messages(messages);

    Ok(response)
}

pub fn quick_buy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stock_id: u64,
    shares: u64,
    slippage: u64,
) -> Result<Response, ContractError> {
    // Validate inputs
    if shares == 0 {
        return Err(ContractError::GenericError(f!("Cannot quick buy 0 shares")));
    }

    // Load the stock and check if it's in sale (auction ended)
    let stock = STOCKS
        .load(deps.storage, &stock_id.to_be_bytes())
        .map_err(|_| ContractError::NotFound(f!("Stock with id {stock_id}")))?;

    let current_timestamp = env.block.time.nanos() / 1_000_000;

    if stock.auction_end.is_none() || stock.auction_end > Some(current_timestamp) {
        return Err(ContractError::GenericError(f!("Stock is not in sale")));
    }

    // Get current price estimation
    // Returns error if shares is more than current sell_volume
    let buy_price = query::orders::get_buy_price(deps.as_ref(), env.clone(), stock_id, shares)?;

    let total_price: u128 = buy_price.total_price.parse().unwrap();

    // Apply slippage to determine max price willing to pay
    let max_price_with_slippage = total_price + (total_price * slippage as u128 / 100);

    // Check if the user sent enough funds
    let sent_funds = info.funds.iter().find(|coin| coin.denom == DENOM);

    if sent_funds.is_none() {
        return Err(ContractError::GenericError(f!("No {DENOM} sent")));
    }

    let sent_amount = sent_funds.unwrap().amount.u128();

    if sent_amount < max_price_with_slippage {
        return Err(ContractError::GenericError(f!(
            "Insufficient funds: sent {sent_amount}, required with slippage {max_price_with_slippage}"
        )));
    }

    // Get sell orders to match with
    let open_sell_orders = query::orders::get_open_sell_orders_by_stock_id(
        deps.as_ref(),
        env.clone(),
        stock_id,
        OrderSort::PriceAsc,
    )?
    .orders;

    let mut remaining_shares = shares;
    let mut messages = vec![];
    let mut actual_cost = 0u128;

    // match sell orders
    for mut sell_order in open_sell_orders {
        let balance = sell_order.available_shares - sell_order.sold_shares;

        let take = std::cmp::min(remaining_shares, balance);

        // Calculate cost for this batch of shares
        let batch_cost = take as u128 * sell_order.price_per_share;

        // Check if we're still within slippage limits
        if actual_cost + batch_cost > max_price_with_slippage {
            return Err(ContractError::GenericError(f!(
                "Price exceeded slippage tolerance"
            )));
        }

        // Update sell order
        sell_order.sold_shares += take;

        if sell_order.sold_shares == sell_order.available_shares {
            sell_order.resolved_at = Some(current_timestamp);
        }

        SELL_ORDERS.save(deps.storage, &sell_order.id.to_be_bytes(), &sell_order)?;

        actual_cost += batch_cost;

        // Transfer share from seller to buyer
        let mut seller_share = query::shares::get_shares_by_owner(
            deps.as_ref(),
            env.clone(),
            sell_order.owner.clone(),
        )?
        .shares
        .into_iter()
        .filter(|share| share.stock_id == stock_id)
        .nth(0)
        .ok_or(ContractError::GenericError("This shouldn't happen".into()))?;

        seller_share.no_of_shares -= take;
        SHARES.save(deps.storage, &seller_share.id.to_be_bytes(), &seller_share)?;

        let buyer_share =
            query::shares::get_shares_by_owner(deps.as_ref(), env.clone(), info.sender.clone())?
                .shares
                .into_iter()
                .filter(|share| share.stock_id == stock_id)
                .nth(0);

        // if buyer already have shares in stock, update share count
        if let Some(mut buyer_share) = buyer_share {
            buyer_share.no_of_shares += take;
            SHARES.save(deps.storage, &buyer_share.id.to_be_bytes(), &buyer_share)?;
        } else {
            // else create new share for buyer
            let new_share_id = SHARE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
            SHARE_COUNT.save(deps.storage, &new_share_id)?;

            let new_share = Share {
                id: new_share_id,
                stock_id,
                no_of_shares: take,
                owner: info.sender.clone(),
            };

            SHARES.save(deps.storage, &new_share_id.to_be_bytes(), &new_share)?;
        }

        // Create Sale record
        let sale_id = SALE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
        SALE_COUNT.save(deps.storage, &sale_id)?;

        let sale = Sale {
            id: sale_id,
            stock_id,
            no_of_shares: take,
            price_per_share: sell_order.price_per_share,
            from: sell_order.owner.clone(),
            to: info.sender.clone(),
            created_at: current_timestamp,
        };

        SALES.save(deps.storage, &sale_id.to_be_bytes(), &sale)?;

        // Send funds to seller
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: sell_order.owner.to_string(),
            amount: coins(batch_cost, DENOM),
        }));

        remaining_shares -= take;
        if remaining_shares == 0 {
            break;
        }
    }

    // Refund excess funds
    let excess_funds = sent_amount - actual_cost;

    if excess_funds > 0 {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(excess_funds, DENOM),
        }));
    }

    let response = Response::new()
        .add_attribute("action", "quick_buy")
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute("cost", actual_cost.to_string())
        .add_attribute(
            "average_price_per_share",
            (actual_cost / shares as u128).to_string(),
        )
        .add_messages(messages);

    Ok(response)
}

pub fn quick_sell(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stock_id: u64,
    shares: u64,
    price_per_share: u128,
    slippage: u64,
) -> Result<Response, ContractError> {
    // Validate inputs
    if shares == 0 {
        return Err(ContractError::GenericError(f!(
            "Cannot quick sell 0 shares"
        )));
    }

    // Load the stock and check if it's in sale (auction ended)
    let stock = STOCKS
        .load(deps.storage, &stock_id.to_be_bytes())
        .map_err(|_| ContractError::NotFound(f!("Stock with id {stock_id}")))?;

    let current_timestamp = env.block.time.nanos() / 1_000_000;

    if stock.auction_end.is_none() || stock.auction_end > Some(current_timestamp) {
        return Err(ContractError::GenericError(f!("Stock is not in sale")));
    }

    // Check if the seller has enough shares
    let mut seller_share =
        query::shares::get_shares_by_owner(deps.as_ref(), env.clone(), info.sender.clone())?
            .shares
            .into_iter()
            .filter(|share| share.stock_id == stock_id)
            .nth(0)
            .ok_or(ContractError::GenericError(
                "You do not have shares in this stock".into(),
            ))?;

    if seller_share.no_of_shares < shares {
        return Err(ContractError::GenericError(f!(
            "Insufficient shares: have {}, need {shares}",
            seller_share.no_of_shares
        )));
    }

    // Get current price estimation
    // Returns error if shares is more than current buy_volume
    let sell_price = query::orders::get_sell_price(deps.as_ref(), env.clone(), stock_id, shares)?;

    let total_price: u128 = sell_price.total_price.parse().unwrap();

    // Apply slippage to determine min price willing to accept
    let total_requested_price = price_per_share * shares as u128;

    let min_price_with_slippage =
        total_requested_price - (total_requested_price * slippage as u128 / 100);

    if total_price < min_price_with_slippage {
        return Err(ContractError::GenericError(f!(
            "Actual price fell below slippage tolerance, please increase slippage"
        )));
    }

    // Get buy orders to match with
    let open_buy_orders = query::orders::get_open_buy_orders_by_stock_id(
        deps.as_ref(),
        env.clone(),
        stock_id,
        OrderSort::PriceDesc,
    )?
    .orders;

    let mut remaining_shares = shares;
    let mut messages = vec![];
    let mut actual_revenue = 0u128;

    // Match buy orders
    for mut buy_order in open_buy_orders {
        let balance = buy_order.requested_shares - buy_order.bought_shares;

        let take = std::cmp::min(remaining_shares, balance);

        // Calculate revenue for this batch of shares
        let batch_revenue = take as u128 * buy_order.price_per_share;

        // Update buy order
        buy_order.bought_shares += take;

        if buy_order.bought_shares == buy_order.requested_shares {
            buy_order.resolved_at = Some(current_timestamp);
        }

        BUY_ORDERS.save(deps.storage, &buy_order.id.to_be_bytes(), &buy_order)?;

        actual_revenue += batch_revenue;

        // Transfer shares from seller to buyer
        seller_share.no_of_shares -= take;
        SHARES.save(deps.storage, &seller_share.id.to_be_bytes(), &seller_share)?;

        let buyer_share = query::shares::get_shares_by_owner(
            deps.as_ref(),
            env.clone(),
            buy_order.owner.clone(),
        )?
        .shares
        .into_iter()
        .filter(|share| share.stock_id == stock_id)
        .nth(0);

        // If buyer already has shares in stock, update share count
        if let Some(mut buyer_share) = buyer_share {
            buyer_share.no_of_shares += take;
            SHARES.save(deps.storage, &buyer_share.id.to_be_bytes(), &buyer_share)?;
        } else {
            // Else create new share for buyer
            let new_share_id = SHARE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
            SHARE_COUNT.save(deps.storage, &new_share_id)?;

            let new_share = Share {
                id: new_share_id,
                stock_id,
                no_of_shares: take,
                owner: buy_order.owner.clone(),
            };

            SHARES.save(deps.storage, &new_share_id.to_be_bytes(), &new_share)?;
        }

        // Create Sale record
        let sale_id = SALE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
        SALE_COUNT.save(deps.storage, &sale_id)?;

        let sale = Sale {
            id: sale_id,
            stock_id,
            no_of_shares: take,
            price_per_share: buy_order.price_per_share,
            from: info.sender.clone(),
            to: buy_order.owner.clone(),
            created_at: current_timestamp,
        };

        SALES.save(deps.storage, &sale_id.to_be_bytes(), &sale)?;

        // Send funds to seller
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(batch_revenue, DENOM),
        }));

        remaining_shares -= take;
        if remaining_shares == 0 {
            break;
        }
    }

    // Verify that all shares were sold
    if remaining_shares > 0 {
        return Err(ContractError::GenericError(f!(
            "Could not sell all shares. {remaining_shares} shares remain unsold."
        )));
    }

    let response = Response::new()
        .add_attribute("action", "quick_sell")
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute("revenue", actual_revenue.to_string())
        .add_attribute(
            "average_price_per_share",
            (actual_revenue / shares as u128).to_string(),
        )
        .add_messages(messages);

    Ok(response)
}
