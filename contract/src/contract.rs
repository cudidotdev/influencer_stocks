#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

pub mod execute;
pub mod query;

// Denomination of the token we're using
pub const DENOM: &str = "uosmo";

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:influencer-stocks";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateStock { ticker } => {
            execute::stocks::create_stock(deps, env, info, ticker)
        }

        ExecuteMsg::StartAuction { stock_id } => {
            execute::stocks::start_auction(deps, env, info, stock_id)
        }

        ExecuteMsg::EndAuction { stock_id } => {
            execute::stocks::end_auction(deps, env, info, stock_id)
        }

        ExecuteMsg::PlaceBid {
            stock_id,
            price_per_share,
            shares,
        } => execute::bids::place_bid(deps, env, info, stock_id, price_per_share, shares),

        ExecuteMsg::CreateBuyOrder {
            stock_id,
            price_per_share,
            shares,
        } => execute::orders::create_buy_order(deps, env, info, stock_id, shares, price_per_share),

        ExecuteMsg::CreateSellOrder {
            stock_id,
            price_per_share,
            shares,
        } => execute::orders::create_sell_order(deps, env, info, stock_id, shares, price_per_share),

        ExecuteMsg::CancelBuyOrder { buy_order_id } => {
            execute::orders::cancel_buy_order(deps, env, info, buy_order_id)
        }

        ExecuteMsg::CancelSellOrder { sell_order_id } => {
            execute::orders::cancel_sell_order(deps, env, info, sell_order_id)
        }

        ExecuteMsg::QuickBuy {
            stock_id,
            shares,
            slippage,
        } => execute::orders::quick_buy(deps, env, info, stock_id, shares, slippage),

        ExecuteMsg::QuickSell {
            stock_id,
            shares,
            slippage,
            price_per_share,
        } => execute::orders::quick_sell(
            deps,
            env,
            info,
            stock_id,
            shares,
            price_per_share,
            slippage,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStockById { stock_id } => {
            to_json_binary(&query::stocks::get_stock_by_id(deps, env, stock_id)?)
        }

        QueryMsg::GetAllStocks {
            start_after,
            in_auction,
            in_sale,
            marked_as_active_auction,
        } => to_json_binary(&query::stocks::get_all_stocks(
            deps,
            env,
            start_after,
            in_auction,
            in_sale,
            marked_as_active_auction,
        )?),

        QueryMsg::GetStocksByInfluencer {
            influencer,
            start_after,
        } => to_json_binary(&query::stocks::get_stocks_by_influencer(
            deps,
            env,
            influencer,
            start_after,
        )?),

        QueryMsg::GetBidById { bid_id } => {
            to_json_binary(&query::bids::get_bid_by_id(deps, env, bid_id)?)
        }

        QueryMsg::GetBidsByBidder {
            bidder,
            is_open,
            is_active,
            stock_id,
        } => to_json_binary(&query::bids::get_bids_by_bidder(
            deps, env, bidder, is_open, is_active, stock_id,
        )?),

        QueryMsg::GetMinimumBidPrice {
            stock_id,
            shares_requested,
        } => to_json_binary(&query::bids::get_minimum_bid_price(
            deps,
            env,
            stock_id,
            shares_requested,
        )?),

        QueryMsg::GetOpenBidsByStock { stock_id } => {
            to_json_binary(&query::bids::get_open_bids_by_stock(deps, env, stock_id)?)
        }

        QueryMsg::GetBidsByStock { stock_id } => {
            to_json_binary(&query::bids::get_bids_by_stock_id(deps, env, stock_id)?)
        }

        QueryMsg::GetShareById { share_id } => {
            to_json_binary(&query::shares::get_shares_by_id(deps, env, share_id)?)
        }

        QueryMsg::GetSharesByOwner { owner } => {
            to_json_binary(&query::shares::get_shares_by_owner(deps, env, owner)?)
        }

        QueryMsg::GetSharesByStock { stock_id } => {
            to_json_binary(&query::shares::get_shares_by_stock_id(deps, env, stock_id)?)
        }

        QueryMsg::GetSellPrice {
            stock_id,
            requested_shares,
        } => to_json_binary(&query::orders::get_sell_price(
            deps,
            env,
            stock_id,
            requested_shares,
        )?),

        QueryMsg::GetBuyPrice {
            stock_id,
            requested_shares,
        } => to_json_binary(&query::orders::get_buy_price(
            deps,
            env,
            stock_id,
            requested_shares,
        )?),

        QueryMsg::GetTotalBuyVolume { stock_id } => to_json_binary(
            &query::orders::get_total_buy_order_volume(deps, env, stock_id)?,
        ),

        QueryMsg::GetTotalSellVolume { stock_id } => to_json_binary(
            &query::orders::get_total_sell_order_volume(deps, env, stock_id)?,
        ),

        QueryMsg::GetOpenBuyOrdersByStock { stock_id, sort_by } => to_json_binary(
            &query::orders::get_open_buy_orders_by_stock_id(deps, env, stock_id, sort_by)?,
        ),

        QueryMsg::GetOpenSellOrdersByStock { stock_id, sort_by } => to_json_binary(
            &query::orders::get_open_sell_orders_by_stock_id(deps, env, stock_id, sort_by)?,
        ),

        QueryMsg::GetOpenBuyOrdersByOwner { owner, sort_by } => to_json_binary(
            &query::orders::get_open_buy_orders_by_owner(deps, env, owner, sort_by)?,
        ),

        QueryMsg::GetOpenSellOrdersByOwner { owner, sort_by } => to_json_binary(
            &query::orders::get_open_sell_orders_by_owner(deps, env, owner, sort_by)?,
        ),

        QueryMsg::GetBuyOrderById { buy_order_id } => to_json_binary(
            &query::orders::get_buy_order_by_id(deps, env, buy_order_id)?,
        ),

        QueryMsg::GetSellOrderById { sell_order_id } => to_json_binary(
            &query::orders::get_sell_order_by_id(deps, env, sell_order_id)?,
        ),

        QueryMsg::GetSalesByStock { stock_id } => {
            to_json_binary(&query::sales::get_sales_by_stock_id(deps, env, stock_id)?)
        }

        QueryMsg::GetSalesByUser { user } => {
            to_json_binary(&query::sales::get_sales_by_user(deps, env, user)?)
        }

        QueryMsg::GetSalesById { sale_id } => {
            to_json_binary(&query::sales::get_sale_by_id(deps, env, sale_id)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let creator = deps.api.addr_make("creator");

        let msg = InstantiateMsg {};

        let info = message_info(&creator, &coins(1000, "token"));

        let sender = info.sender.clone();

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert!(utils::response::contains_attribute(
            &res,
            "method",
            "instantiate"
        ));

        assert!(utils::response::contains_attribute(
            &res,
            "owner",
            sender.as_str()
        ));
    }
}
