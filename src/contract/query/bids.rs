use cosmwasm_std::{Deps, Order, StdResult};

use crate::state::BIDS;

// Minimum bid increment (0.000001 huahua)
const MIN_BID_INCREMENT: u128 = 1;

// Function to get the minimum bid price for a given number of shares
pub fn get_minimum_bid_price(deps: Deps, stock_id: u64, requested_shares: u64) -> StdResult<u128> {
    // Default minimum price is 0
    let mut min_price: u128 = 0;
    let mut available_shares = requested_shares;

    // Get all active bids for this stock, ordered by price (descending)
    let active_bids = BIDS
        .idx
        .stock_id
        .prefix(stock_id)
        .range(deps.storage, None, None, Order::Descending)
        .filter(|item| match item {
            Ok((_, bid)) => !bid.outbid,
            _ => false,
        })
        .collect::<StdResult<Vec<_>>>()?;

    // Start with highest priced bids and work down
    for (_, bid) in active_bids {
        if available_shares <= bid.shares_requested {
            // We need to outbid this price
            min_price = bid.price_per_share + MIN_BID_INCREMENT;
            break;
        } else {
            // We can skip this bid and move to the next one
            available_shares -= bid.shares_requested;
        }
    }

    Ok(min_price)
}
