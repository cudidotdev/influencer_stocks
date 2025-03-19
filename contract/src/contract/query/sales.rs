use cosmwasm_std::{Addr, Deps, Env, Order, StdError, StdResult};

use crate::{
    msg::{GetSaleByIdResponse, GetSalesByUserResponse, GetSalesResponse},
    state::SALES,
};

use format as f;

pub fn get_sale_by_id(deps: Deps, _env: Env, sale_id: u64) -> StdResult<GetSaleByIdResponse> {
    let sale = SALES
        .load(deps.storage, &sale_id.to_be_bytes())
        .map_err(|_| StdError::not_found(f!("Sale with id {sale_id}")))?;

    Ok(GetSaleByIdResponse { sale })
}

pub fn get_sales_by_user(deps: Deps, _env: Env, user: Addr) -> StdResult<GetSalesByUserResponse> {
    let mut buy = SALES
        .idx
        .to
        .prefix(user.clone())
        .range(deps.storage, None, None, Order::Descending)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, bid)| Ok(bid)))
        .collect::<Result<Vec<_>, _>>()?;

    let mut sell = SALES
        .idx
        .from
        .prefix(user)
        .range(deps.storage, None, None, Order::Descending)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, bid)| Ok(bid)))
        .collect::<Result<Vec<_>, _>>()?;

    // order by created_at (descending)
    buy.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    sell.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(GetSalesByUserResponse { buy, sell })
}

pub fn get_sales_by_stock_id(deps: Deps, _env: Env, stock_id: u64) -> StdResult<GetSalesResponse> {
    let sales = SALES
        .idx
        .stock_id
        .prefix(stock_id)
        .range(deps.storage, None, None, Order::Descending)
        // Extract the stock data from each item.
        .map(|item| item.and_then(|(_, bid)| Ok(bid)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GetSalesResponse { sales })
}
