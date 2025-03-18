use cosmwasm_std::{Addr, Deps, Env, Order, StdError, StdResult};

use crate::{
    msg::{GetBidByIdResponse, GetBidsResponse, GetMinimumBidPriceResponse},
    state::BIDS,
};

use format as f;

pub fn get_bid_by_id(deps: Deps, _env: Env, bid_id: u64) -> StdResult<GetBidByIdResponse> {
    let bid = BIDS
        .load(deps.storage, &bid_id.to_be_bytes())
        .map_err(|_| StdError::not_found(f!("Bid with id {bid_id}")))?;

    Ok(GetBidByIdResponse { bid })
}

pub fn get_bids_by_bidder(
    deps: Deps,
    _env: Env,
    bidder: Addr,
    is_open: Option<bool>,
    is_active: Option<bool>,
    stock_id: Option<u64>,
) -> StdResult<GetBidsResponse> {
    // Query BIDS by bidder in descending order based on their id,
    let mut bids = BIDS
        .idx
        .bidder
        .prefix(bidder)
        .range(deps.storage, None, None, Order::Descending)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, bid)| Ok(bid)))
        .collect::<Result<Vec<_>, _>>()?;

    // Filter by stock_id
    if let Some(stock_id) = stock_id {
        bids = bids
            .into_iter()
            .filter(|bid| bid.stock_id == stock_id)
            .collect();
    }

    // Filter by open
    if let Some(is_open) = is_open {
        if is_open {
            bids = bids.into_iter().filter(|bid| bid.open == 1).collect();
        } else {
            bids = bids.into_iter().filter(|bid| bid.open == 0).collect();
        }
    }

    // Filter by active
    if let Some(is_active) = is_active {
        bids = bids
            .into_iter()
            .filter(|bid| bid.active == is_active)
            .collect();
    }

    Ok(GetBidsResponse { bids })
}

pub fn get_open_bids_by_stock(deps: Deps, _env: Env, stock_id: u64) -> StdResult<GetBidsResponse> {
    // This filters by stock(stock_id) and open(1)
    // while ordering by price in ascending order

    let mut bids = BIDS
        .idx
        .stock_open
        .prefix((stock_id, 1))
        .range(deps.storage, None, None, Order::Ascending)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, bid)| Ok(bid)))
        .collect::<Result<Vec<_>, _>>()?;

    // Then sort in memory by price
    bids.sort_by(|a, b| a.price_per_share.cmp(&b.price_per_share)); // ascending order

    Ok(GetBidsResponse { bids })
}

// Minimum bid increment (0.000001 huahua)
const MIN_BID_INCREMENT: u128 = 1;

// Function to get the minimum bid price for a given number of shares
pub fn get_minimum_bid_price(
    deps: Deps,
    env: Env,
    stock_id: u64,
    shares_requested: u64,
) -> StdResult<GetMinimumBidPriceResponse> {
    // Default minimum price is 0
    let mut min_price: u128 = 0;
    let mut available_shares = shares_requested;

    // Get all open bids for this stock, ordered by price (ascending)
    let open_bids = get_open_bids_by_stock(deps, env, stock_id)?.bids;

    // Start with loweset priced bids and work up
    for bid in open_bids {
        if available_shares <= bid.remaining_shares {
            // We need to outbid this price
            min_price = bid.price_per_share + MIN_BID_INCREMENT;
            break;
        } else {
            // We can skip this bid and move to the next one
            available_shares -= bid.remaining_shares;
        }
    }

    Ok(GetMinimumBidPriceResponse {
        min_price: min_price.to_string(),
        shares_requested,
    })
}

pub fn get_bids_by_stock_id(deps: Deps, _env: Env, stock_id: u64) -> StdResult<GetBidsResponse> {
    let bids = BIDS
        .idx
        .stock_id
        .prefix(stock_id)
        .range(deps.storage, None, None, Order::Descending)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, bid)| Ok(bid)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GetBidsResponse { bids })
}
