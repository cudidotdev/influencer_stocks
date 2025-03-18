use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    contract::query,
    state::{Bid, Share, Stock, BIDS, BID_COUNT, SHARES, SHARE_COUNT, STATE, STOCKS, STOCK_COUNT},
    ContractError,
};

use format as f;

// total shares for a stock fixed at 1_000_000 for now,
// may be updated to vary based on creators choice
pub const TOTAL_SHARES: u64 = 1_000_000;

pub fn create_stock(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ticker: String,
) -> Result<Response, ContractError> {
    // Validate addresses
    let influencer_addr = info.sender.clone();

    // increment and save stock_id counter
    let stock_id = STOCK_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
    STOCK_COUNT.save(deps.storage, &stock_id)?;

    let created_at = env.block.time.nanos() / 1_000_000;

    // Create new stock
    let stock = Stock {
        id: stock_id,
        ticker,
        influencer: influencer_addr,
        total_shares: TOTAL_SHARES,
        auction_start: None,
        auction_end: None,
        created_at,
    };

    // Save the stock
    STOCKS.save(deps.storage, stock_id.to_be_bytes().as_slice(), &stock)?;

    Ok(Response::new()
        .add_attribute("action", "create_stock")
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("influencer", info.sender)
        .add_attribute("created_at", created_at.to_string())
        .add_attribute("total_shares", TOTAL_SHARES.to_string()))
}

pub fn start_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stock_id: u64,
) -> Result<Response, ContractError> {
    // Convert stock_id from u64 into be_bytes
    let stock_id_bytes = stock_id.to_be_bytes();

    //get stock
    let mut stock = STOCKS
        .load(deps.storage, &stock_id_bytes)
        .map_err(|_| ContractError::NotFound(f!("Stock with id {stock_id}")))?;

    // if stock doesn't belong to sender
    // return an unauthorized error
    if stock.influencer != info.sender.clone() {
        return Err(ContractError::Unauthorized);
    }

    // Get current blockchain time in milliseconds
    let current_time = env.block.time.nanos() / 1_000_000; // Convert nanos to millis

    // Check auction is ended and stock is in sale
    if let Some(end_timestamp) = stock.auction_end {
        if current_time > end_timestamp {
            return Err(ContractError::GenericError(f!(
                "Stock has already been auctioned and in sale"
            )));
        }
    }

    // If stock is been auctioned
    if stock.auction_start.is_some() {
        return Err(ContractError::GenericError(f!(
            "Stock is already been auctioned"
        )));
    }

    // Set auction start time to current blockchain time
    let start_timestamp = current_time;
    stock.auction_start = Some(current_time);

    // Calculate auction end time (24 hours later)
    let end_timestamp = start_timestamp + (24 * 60 * 60 * 1000); // 24 hours in milliseconds
    stock.auction_end = Some(end_timestamp);

    // Save updated stock
    STOCKS.save(deps.storage, &stock_id_bytes, &stock)?;

    // Place inital bid with price set as 0
    let bid_id = BID_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
    BID_COUNT.save(deps.storage, &bid_id)?;

    let bid = Bid {
        id: bid_id,
        stock_id,
        bidder: info.sender.clone(),
        price_per_share: 0,
        shares_requested: stock.total_shares,
        remaining_shares: stock.total_shares,
        created_at: current_time,
        open: 1,
        active: true,
    };

    // Save the bid
    BIDS.save(deps.storage, &bid_id.to_be_bytes(), &bid)?;

    Ok(Response::new()
        .add_attribute("action", "start_auction")
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("auction_start", start_timestamp.to_string())
        .add_attribute("auction_end", end_timestamp.to_string()))
}

pub fn end_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stock_id: u64,
) -> Result<Response, ContractError> {
    let stock_id_bytes = stock_id.to_be_bytes();

    let mut stock = STOCKS
        .load(deps.storage, &stock_id_bytes)
        .map_err(|_| ContractError::NotFound(f!("Stock with id {stock_id}")))?;

    // Get contract state to check for owner
    let state = STATE.load(deps.storage)?;

    // Authorization check - allow both the owner and the influencer to end auction
    let is_owner = state.owner == info.sender;
    let is_influencer = stock.influencer == info.sender;

    if !is_owner && !is_influencer {
        return Err(ContractError::Unauthorized);
    }

    // Get current blockchain time in milliseconds
    let current_time = env.block.time.nanos() / 1_000_000; // Convert nanos to millis

    // Check if auction is ended and stock is in sale
    if let Some(end_timestamp) = stock.auction_end {
        if current_time > end_timestamp {
            return Err(ContractError::GenericError(f!(
                "Stock has already been auctioned and in sale"
            )));
        }
    }

    // Check stock hasn't started auction
    if stock.auction_start.is_none() {
        return Err(ContractError::GenericError(f!(
            "Stock is yet to be auctioned"
        )));
    }

    // Update auction end time to current block time
    stock.auction_end = Some(current_time);

    // End the auction
    STOCKS.save(deps.storage, &stock_id_bytes, &stock)?;

    // Create stakes from winning (open) bids
    let open_bids = query::bids::get_open_bids_by_stock(deps.as_ref(), env.clone(), stock_id)?.bids;

    for mut bid in open_bids {
        // Create a stake from bid
        let share_id = SHARE_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
        SHARE_COUNT.save(deps.storage, &share_id)?;

        let share = Share {
            id: share_id,
            stock_id,
            no_of_shares: bid.remaining_shares,
            owner: bid.bidder.clone(),
        };

        SHARES.save(deps.storage, &share_id.to_be_bytes(), &share)?;

        // close bid
        bid.open = 0;

        BIDS.save(deps.storage, &bid.id.to_be_bytes(), &bid)?;
    }

    // Make all bids for the stock inactive
    let all_bids = query::bids::get_bids_by_stock_id(deps.as_ref(), env.clone(), stock_id)?.bids;

    for mut bid in all_bids {
        bid.active = false;

        BIDS.save(deps.storage, &bid.id.to_be_bytes(), &bid)?;
    }

    Ok(Response::new()
        .add_attribute("action", "end_auction")
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("ended_by", info.sender.to_string())
        .add_attribute("ended_at", current_time.to_string())
        .add_attribute("is_influencer", is_influencer.to_string()))
}
