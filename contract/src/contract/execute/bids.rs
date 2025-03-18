use cosmwasm_std::{coins, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response};

use crate::{
    contract::{query, DENOM},
    state::{Bid, BIDS, BID_COUNT, STOCKS},
    ContractError,
};

use format as f;

pub fn place_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stock_id: u64,
    price_per_share: u128,
    shares: u64,
) -> Result<Response, ContractError> {
    // Check if funds are sent
    let sent_funds = info.funds.iter().find(|coin| coin.denom == DENOM);

    if sent_funds.is_none() {
        return Err(ContractError::GenericError(f!("No {DENOM} sent")));
    }

    let sent_amount = sent_funds.unwrap().amount.u128();
    let expected_amount = price_per_share * shares as u128;

    // Validate funds
    if sent_amount < expected_amount {
        return Err(ContractError::GenericError(f!(
            "Insufficient funds sent. Expected {expected_amount}, got {sent_amount}"
        )));
    }

    // Load the stock
    let stock_id_bytes = stock_id.to_be_bytes();
    let stock = STOCKS
        .load(deps.storage, &stock_id_bytes)
        .map_err(|_| ContractError::NotFound(f!("Stock with id {stock_id}")))?;

    // Check if auction has expired
    let current_time = env.block.time.nanos() / 1_000_000;
    if let Some(end_timestamp) = stock.auction_end {
        if current_time > end_timestamp {
            return Err(ContractError::GenericError(f!("Auction has ended")));
        }
    }

    // Check if auction hasn't started
    if stock.auction_start.is_none() {
        return Err(ContractError::GenericError(f!(
            "Stock is yet to be auctioned"
        )));
    }

    // Get the current minimum bid price
    let min_bid_price =
        query::bids::get_minimum_bid_price(deps.as_ref(), env.clone(), stock_id, shares)?
            .min_price
            .parse()
            .map_err(|_| ContractError::GenericError(f!("Price conversion error")))?;

    // Check if the bid price is greater than or equal to the minimum bid price
    if price_per_share < min_bid_price {
        return Err(ContractError::GenericError(f!(
            "Bid price too low. Minimum price is {min_bid_price}"
        )));
    }

    // Create a new bid
    let bid_id = BID_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
    BID_COUNT.save(deps.storage, &bid_id)?;

    let bid = Bid {
        id: bid_id,
        stock_id,
        bidder: info.sender.clone(),
        price_per_share,
        shares_requested: shares,
        remaining_shares: shares,
        created_at: current_time,
        open: 1,
        active: true,
    };

    // Save the bid
    BIDS.save(deps.storage, &bid_id.to_be_bytes(), &bid)?;

    // amount to send to influencer wallet
    let mut influencer_pay = expected_amount;

    // messages for funds disbursement
    let mut messages = vec![];

    // calculate excess amount sent by the user
    let excess_amount = if sent_amount > expected_amount {
        sent_amount - expected_amount
    } else {
        0
    };

    // If there's excess, send it back to the sender
    if excess_amount > 0 {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(excess_amount, DENOM),
        }));
    }

    // Process outbidding
    let outbids = process_outbids(deps, env, bid_id, stock_id, shares)?;

    // Refund outbids
    for outbid in outbids {
        if outbid.1 > 0 {
            // subtract refund from influencer's pay
            influencer_pay -= outbid.1;

            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: outbid.0.to_string(),
                amount: coins(outbid.1, DENOM),
            }));
        }
    }

    // Transfer the influencer's pay to influencer
    if influencer_pay > 0 {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: stock.influencer.to_string(),
            amount: coins(influencer_pay, DENOM),
        }));
    }

    Ok(Response::new()
        .add_attribute("action", "place_bid")
        .add_attribute("bid_id", bid_id.to_string())
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("bidder", info.sender.to_string())
        .add_attribute("price_per_share", price_per_share.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute(
            "total_amount",
            (price_per_share * shares as u128).to_string(),
        )
        .add_messages(messages))
}

fn process_outbids(
    deps: DepsMut,
    env: Env,
    bid_id: u64,
    stock_id: u64,
    shares_requested: u64,
) -> Result<Vec<(Addr, u128)>, ContractError> {
    let mut available_shares = shares_requested;

    // Get all open bids for this stock, ordered by price (ascending)
    let open_bids = query::bids::get_open_bids_by_stock(deps.as_ref(), env, stock_id)?.bids;

    let mut outbids = Vec::new();

    // Start with loweset priced bids and work up
    for mut bid in open_bids {
        // Exclude the newly created bid
        if bid.id == bid_id {
            continue;
        }

        if available_shares <= bid.remaining_shares {
            // calculate remaining_shares and add to outbids
            let remaining_shares = bid.remaining_shares - available_shares;

            outbids.push((
                bid.bidder.clone(),
                bid.price_per_share * available_shares as u128,
            ));

            // update remaining_shares
            bid.remaining_shares = remaining_shares;
            BIDS.save(deps.storage, &bid.id.to_be_bytes(), &bid)?;

            break;
        } else {
            // update available_shares
            available_shares -= bid.remaining_shares;

            // add to outbid
            outbids.push((
                bid.bidder.clone(),
                bid.price_per_share * bid.remaining_shares as u128,
            ));

            // updated remaining_shares
            bid.remaining_shares = 0;

            // close bid
            bid.open = 0;

            // save
            BIDS.save(deps.storage, &bid.id.to_be_bytes(), &bid)?;
        }
    }

    Ok(outbids)
}
