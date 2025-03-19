use crate::msg::{
    GetBuyOrderByIdResponse, GetBuyOrdersResponse, GetBuyPriceResponse, GetSellOrderByIdResponse,
    GetSellOrdersResponse, GetSellPriceResponse, GetTotalBuyVolumeResponse,
    GetTotalSellVolumeResponse, OrderSort,
};
use crate::state::{BUY_ORDERS, SELL_ORDERS};
use cosmwasm_std::{Addr, Deps, Env, Order, StdError, StdResult};

use format as f;

pub fn get_open_sell_orders_by_stock_id(
    deps: Deps,
    _env: Env,
    stock_id: u64,
    sort_by: OrderSort,
) -> StdResult<GetSellOrdersResponse> {
    // Get orders by stock_id
    let mut orders = SELL_ORDERS
        .idx
        .stock_id
        .prefix(stock_id)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        // filter by open orders
        .filter_map(|item| {
            if let Ok((_, order)) = item {
                if order.resolved_at.is_none() {
                    Some(order)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    match sort_by {
        // sort by date created (ascending)
        OrderSort::CreatedAtAsc => orders.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
        // sort by date created (descending)
        OrderSort::CreatedAtDesc => orders.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
        // sort by price created (ascending)
        OrderSort::PriceAsc => orders.sort_by(|a, b| a.price_per_share.cmp(&b.price_per_share)),
        // sort by price created (descending)
        OrderSort::PriceDesc => orders.sort_by(|a, b| b.price_per_share.cmp(&a.price_per_share)),
    }

    Ok(GetSellOrdersResponse { orders })
}

pub fn get_open_buy_orders_by_stock_id(
    deps: Deps,
    _env: Env,
    stock_id: u64,
    sort_by: OrderSort,
) -> StdResult<GetBuyOrdersResponse> {
    // Get orders by stock_id
    let mut orders = BUY_ORDERS
        .idx
        .stock_id
        .prefix(stock_id)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        // filter by open orders
        .filter_map(|item| {
            if let Ok((_, order)) = item {
                if order.resolved_at.is_none() {
                    Some(order)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    match sort_by {
        // sort by date created (ascending)
        OrderSort::CreatedAtAsc => orders.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
        // sort by date created (descending)
        OrderSort::CreatedAtDesc => orders.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
        // sort by price created (ascending)
        OrderSort::PriceAsc => orders.sort_by(|a, b| a.price_per_share.cmp(&b.price_per_share)),
        // sort by price created (descending)
        OrderSort::PriceDesc => orders.sort_by(|a, b| b.price_per_share.cmp(&a.price_per_share)),
    }

    Ok(GetBuyOrdersResponse { orders })
}

pub fn get_sell_order_by_id(
    deps: Deps,
    _env: Env,
    sell_order_id: u64,
) -> StdResult<GetSellOrderByIdResponse> {
    let order = SELL_ORDERS
        .load(deps.storage, &sell_order_id.to_be_bytes())
        .map_err(|_| StdError::not_found(f!("Sell order with id {sell_order_id}")))?;

    Ok(GetSellOrderByIdResponse { order })
}

pub fn get_buy_order_by_id(
    deps: Deps,
    _env: Env,
    buy_order_id: u64,
) -> StdResult<GetBuyOrderByIdResponse> {
    let order = BUY_ORDERS
        .load(deps.storage, &buy_order_id.to_be_bytes())
        .map_err(|_| StdError::not_found(f!("Buy order with id {buy_order_id}")))?;

    Ok(GetBuyOrderByIdResponse { order })
}

pub fn get_open_sell_orders_by_owner(
    deps: Deps,
    _env: Env,
    owner: Addr,
    sort_by: OrderSort,
) -> StdResult<GetSellOrdersResponse> {
    // Query BIDS by bidder in descending order based on their id,
    let mut orders = SELL_ORDERS
        .idx
        .owner
        .prefix(owner)
        .range(deps.storage, None, None, Order::Descending)
        // filter by open orders
        .filter_map(|item| {
            if let Ok((_, order)) = item {
                if order.resolved_at.is_none() {
                    Some(order)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    match sort_by {
        // sort by date created (ascending)
        OrderSort::CreatedAtAsc => orders.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
        // sort by date created (descending)
        OrderSort::CreatedAtDesc => orders.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
        // sort by price created (ascending)
        OrderSort::PriceAsc => orders.sort_by(|a, b| a.price_per_share.cmp(&b.price_per_share)),
        // sort by price created (descending)
        OrderSort::PriceDesc => orders.sort_by(|a, b| b.price_per_share.cmp(&a.price_per_share)),
    }

    Ok(GetSellOrdersResponse { orders })
}

pub fn get_open_buy_orders_by_owner(
    deps: Deps,
    _env: Env,
    owner: Addr,
    sort_by: OrderSort,
) -> StdResult<GetBuyOrdersResponse> {
    // Query BIDS by bidder in descending order based on their id,
    let mut orders = BUY_ORDERS
        .idx
        .owner
        .prefix(owner)
        .range(deps.storage, None, None, Order::Descending)
        // filter by open orders
        .filter_map(|item| {
            if let Ok((_, order)) = item {
                if order.resolved_at.is_none() {
                    Some(order)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    match sort_by {
        // sort by date created (ascending)
        OrderSort::CreatedAtAsc => orders.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
        // sort by date created (descending)
        OrderSort::CreatedAtDesc => orders.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
        // sort by price created (ascending)
        OrderSort::PriceAsc => orders.sort_by(|a, b| a.price_per_share.cmp(&b.price_per_share)),
        // sort by price created (descending)
        OrderSort::PriceDesc => orders.sort_by(|a, b| b.price_per_share.cmp(&a.price_per_share)),
    }

    Ok(GetBuyOrdersResponse { orders })
}

pub fn get_total_sell_order_volume(
    deps: Deps,
    env: Env,
    stock_id: u64,
) -> StdResult<GetTotalSellVolumeResponse> {
    let open_sell_orders =
        get_open_sell_orders_by_stock_id(deps, env, stock_id, OrderSort::PriceAsc)?.orders;

    let total_available_shares = open_sell_orders.iter().fold(0u64, |acc, order| {
        acc + (order.available_shares - order.sold_shares)
    });

    Ok(GetTotalSellVolumeResponse {
        amount: total_available_shares,
    })
}

pub fn get_total_buy_order_volume(
    deps: Deps,
    env: Env,
    stock_id: u64,
) -> StdResult<GetTotalBuyVolumeResponse> {
    let open_buy_orders =
        get_open_buy_orders_by_stock_id(deps, env, stock_id, OrderSort::PriceDesc)?.orders;

    let total_available_shares = open_buy_orders.iter().fold(0u64, |acc, order| {
        acc + (order.requested_shares - order.bought_shares)
    });

    Ok(GetTotalBuyVolumeResponse {
        amount: total_available_shares,
    })
}

pub fn get_buy_price(
    deps: Deps,
    env: Env,
    stock_id: u64,
    requested_shares: u64,
) -> StdResult<GetBuyPriceResponse> {
    let open_sell_orders =
        get_open_sell_orders_by_stock_id(deps, env.clone(), stock_id, OrderSort::PriceAsc)?.orders;

    // Check if there are enough shares available for sale
    let available_volume = get_total_sell_order_volume(deps, env.clone(), stock_id)?.amount;

    if available_volume < requested_shares {
        return Err(StdError::generic_err(f!(
            "Not enough sell orders to fulfill buy request for {requested_shares} shares. Only {available_volume} shares available."
        )));
    }

    let mut remaining_shares = requested_shares;
    let mut total_price = 0u128;

    // To buy shares, we need to match with sell orders, starting from the lowest price
    for order in open_sell_orders {
        if remaining_shares == 0 {
            break;
        }

        let shares_from_this_order =
            std::cmp::min(remaining_shares, order.available_shares - order.sold_shares);
        let price_from_this_order = shares_from_this_order as u128 * order.price_per_share as u128;

        total_price += price_from_this_order;
        remaining_shares -= shares_from_this_order;
    }

    let price_per_share = if requested_shares > 0 {
        (total_price / requested_shares as u128) as u64
    } else {
        0
    };

    Ok(GetBuyPriceResponse {
        total_price: total_price.to_string(),
        requested_shares,
        price_per_share: price_per_share.to_string(),
    })
}

pub fn get_sell_price(
    deps: Deps,
    env: Env,
    stock_id: u64,
    requested_shares: u64,
) -> StdResult<GetSellPriceResponse> {
    let open_buy_orders =
        get_open_buy_orders_by_stock_id(deps, env.clone(), stock_id, OrderSort::PriceDesc)?.orders;

    // Check if there are enough shares available to buy
    let available_volume = get_total_buy_order_volume(deps, env.clone(), stock_id)?.amount;

    if available_volume < requested_shares {
        return Err(StdError::generic_err(f!(
            "Not enough buy orders to fulfill sell request for {requested_shares} shares. Only {available_volume} shares in demand."
        )));
    }

    let mut remaining_shares = requested_shares;
    let mut total_price = 0u128;

    // To sell shares, we need to match with buy orders, starting from the highest price
    for order in open_buy_orders {
        if remaining_shares == 0 {
            break;
        }

        let shares_from_this_order = std::cmp::min(
            remaining_shares,
            order.requested_shares - order.bought_shares,
        );
        let price_from_this_order = shares_from_this_order as u128 * order.price_per_share as u128;

        total_price += price_from_this_order;
        remaining_shares -= shares_from_this_order;
    }

    let price_per_share = if requested_shares > 0 {
        (total_price / requested_shares as u128) as u64
    } else {
        0
    };

    Ok(GetSellPriceResponse {
        requested_shares,
        total_price: total_price.to_string(),
        price_per_share: price_per_share.to_string(),
    })
}
