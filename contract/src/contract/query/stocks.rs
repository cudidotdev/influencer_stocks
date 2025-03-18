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

const DEFAULT_LIMIT: usize = 20;
const MAX_LIMIT: usize = 30;

pub fn get_all_stocks(
    deps: Deps,
    _env: Env,
    limit: Option<usize>,
    start_after: Option<u64>,
) -> StdResult<GetStocksResponse> {
    // Apply limit to prevent large dataset queries, capped at MAX_LIMI
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);

    // Set starting point for the query
    // (where the next stock_id should start from, excluding the `start_after` id)
    let start = start_after.map(|id| Bound::ExclusiveRaw(id.to_be_bytes().to_vec()));

    // Query STOCKS in descending order based on their id,
    // apply limit, and collect results
    let stocks = STOCKS
        // order is descending, so start becomes maximum
        .range(deps.storage, None, start, Order::Descending)
        .take(limit)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, stock)| Ok(stock)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GetStocksResponse { stocks })
}

pub fn get_stocks_by_influencer(
    deps: Deps,
    _env: Env,
    influencer: Addr,
    limit: Option<usize>,
    start_after: Option<u64>,
) -> StdResult<GetStocksResponse> {
    // Apply limit to prevent large dataset queries, capped at MAX_LIMI
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);

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
        .take(limit)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, stock)| Ok(stock)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GetStocksResponse { stocks })
}

pub fn get_active_auctions(
    deps: Deps,
    _env: Env,
    limit: Option<usize>,
    start_after: Option<u64>,
) -> StdResult<GetStocksResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
    let start = start_after.map(|id| Bound::ExclusiveRaw(id.to_be_bytes().to_vec()));

    let stocks = STOCKS
        .idx
        .auction_active
        .prefix(1u8)
        .range(deps.storage, None, start, Order::Descending)
        .take(limit)
        .map(|item| item.and_then(|(_, stock)| Ok(stock)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GetStocksResponse { stocks })
}

pub fn get_expired_active_auctions(
    deps: Deps,
    env: Env,
    limit: Option<usize>,
    start_after: Option<u64>,
) -> StdResult<GetStocksResponse> {
    let current_time = env.block.time.nanos() / 1_000_000;

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
    let start = start_after.map(|id| Bound::ExclusiveRaw(id.to_be_bytes().to_vec()));

    let expired_active_stocks = STOCKS
        .idx
        .auction_active
        .prefix(1u8)
        .range(deps.storage, None, start, Order::Descending)
        .take(limit)
        .map(|item| item.and_then(|(_, stock)| Ok(stock)))
        .filter(|stock| {
            if let Ok(stock) = stock {
                stock
                    .auction_end
                    .map(|end| end <= current_time)
                    .unwrap_or(false)
            } else {
                false
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GetStocksResponse {
        stocks: expired_active_stocks,
    })
}
