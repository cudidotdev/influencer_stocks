use cosmwasm_std::{Deps, Env, StdError, StdResult};

use crate::{msg::GetStockByIdResponse, state::STOCKS};

use format as f;

pub fn get_stock_by_id(deps: Deps, _env: Env, stock_id: u64) -> StdResult<GetStockByIdResponse> {
    let stock = STOCKS
        .load(deps.storage, &stock_id.to_be_bytes())
        .map_err(|_| StdError::not_found(f!("Stock with id {stock_id}")))?;

    Ok(GetStockByIdResponse { stock })
}
