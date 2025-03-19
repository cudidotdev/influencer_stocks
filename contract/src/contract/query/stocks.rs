use cosmwasm_std::{Addr, Deps, Env, Order, StdError, StdResult};
use cw_storage_plus::Bound;

use crate::{
    msg::{GetStockByIdResponse, GetStocksResponse},
    state::STOCKS,
};

use format as f;

pub fn get_stock_by_id(deps: Deps, _env: Env, stock_id: u64) -> StdResult<GetStockByIdResponse> {
    let stock = STOCKS
        .load(deps.storage, &stock_id.to_be_bytes())
        .map_err(|_| StdError::not_found(f!("Stock with id {stock_id}")))?;

    Ok(GetStockByIdResponse { stock })
}

pub fn get_all_stocks(
    deps: Deps,
    env: Env,
    start_after: Option<u64>,
    in_auction: Option<bool>,
    in_sale: Option<bool>,
    marked_as_active_auction: Option<bool>,
) -> StdResult<GetStocksResponse> {
    // Set starting point for the query
    // (where the next stock_id should start from, excluding the `start_after` id)
    let start = start_after.map(|id| Bound::ExclusiveRaw(id.to_be_bytes().to_vec()));

    // Query STOCKS in descending order based on their id,
    // apply limit, and collect results
    let mut stocks = STOCKS
        // order is descending, so start becomes maximum
        .range(deps.storage, None, start, Order::Descending)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, stock)| Ok(stock)))
        .collect::<Result<Vec<_>, _>>()?;

    // Get current blockchain time in milliseconds
    let current_time = env.block.time.nanos() / 1_000_000; // Convert nanos to millis

    // Filter by auction status
    if let Some(in_auction) = in_auction {
        if in_auction {
            stocks = stocks
                .into_iter()
                .filter(|stock| {
                    // If auction has started but has not ended
                    stock.auction_start.is_some() && stock.auction_end > Some(current_time)
                })
                .collect();
        } else {
            stocks = stocks
                .into_iter()
                .filter(|stock| {
                    // If auction has not started or auction has ended
                    !(stock.auction_start.is_some() && stock.auction_end > Some(current_time))
                })
                .collect();
        }
    }

    // Filter by sales status
    if let Some(in_sale) = in_sale {
        if in_sale {
            stocks = stocks
                .into_iter()
                // If auction has ended
                .filter(|stock| {
                    stock.auction_end.is_some() && stock.auction_end <= Some(current_time)
                })
                .collect();
        } else {
            stocks = stocks
                .into_iter()
                // If auction not ended
                .filter(|stock| {
                    !(stock.auction_end.is_some() && stock.auction_end <= Some(current_time))
                })
                .collect();
        }
    }

    // Filter by marked_as_active_auction
    if let Some(active) = marked_as_active_auction {
        stocks = stocks
            .into_iter()
            // If auction has ended
            .filter(|stock| stock.marked_as_active_auction == active)
            .collect();
    }

    Ok(GetStocksResponse { stocks })
}

pub fn get_stocks_by_influencer(
    deps: Deps,
    _env: Env,
    influencer: Addr,
    start_after: Option<u64>,
) -> StdResult<GetStocksResponse> {
    // Set starting point for the query
    // (where the next stock_id should start from, excluding the `start_after` id)
    let start = start_after.map(|id| Bound::ExclusiveRaw(id.to_be_bytes().to_vec()));

    // Query STOCKS in descending order based on their id,
    // apply limit, and collect results
    let stocks = STOCKS
        // filter to stocks that belongs to the specified influencer
        .idx
        .influencer
        .prefix(influencer)
        // order is descending, so start becomes maximum
        .range(deps.storage, None, start, Order::Descending)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, stock)| Ok(stock)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GetStocksResponse { stocks })
}
