use cosmwasm_std::{Addr, Deps, Env, Order, StdError, StdResult};

use crate::{
    msg::{GetShareByIdResponse, GetSharesResponse},
    state::SHARES,
};

use format as f;

pub fn get_shares_by_stock_id(
    deps: Deps,
    _env: Env,
    stock_id: u64,
) -> StdResult<GetSharesResponse> {
    // Query Shares by stock_id in descending order based on the no_of_shares,

    let mut shares = SHARES
        // filter to stocks that belongs to the specified influencer
        .idx
        .stock_id
        .prefix(stock_id)
        .range(deps.storage, None, None, Order::Ascending)
        // Extract the share data from each item.
        .map(|item| item.and_then(|(_, share)| Ok(share)))
        .collect::<Result<Vec<_>, _>>()?;

    shares.sort_by(|a, b| b.no_of_shares.cmp(&a.no_of_shares));

    Ok(GetSharesResponse { shares })
}

pub fn get_shares_by_owner(deps: Deps, _env: Env, owner: Addr) -> StdResult<GetSharesResponse> {
    let shares = SHARES
        // filter to stocks that belongs to the specified influencer
        .idx
        .owner
        .prefix(owner)
        .range(deps.storage, None, None, Order::Descending)
        // Extract the share data from each item.
        .map(|item| item.and_then(|(_, share)| Ok(share)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GetSharesResponse { shares })
}

pub fn get_shares_by_id(deps: Deps, _env: Env, share_id: u64) -> StdResult<GetShareByIdResponse> {
    let share = SHARES
        .load(deps.storage, &share_id.to_be_bytes())
        .map_err(|_| StdError::not_found(f!("Shares with id {share_id}")))?;

    Ok(GetShareByIdResponse { share })
}
