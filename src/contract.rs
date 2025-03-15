#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

pub mod execute;
pub mod query;

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
        ExecuteMsg::CreateStock { ticker } => execute::create_stock(deps, env, info, ticker),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStockById { stock_id } => {
            to_json_binary(&query::get_stock_by_id(deps, env, stock_id)?)
        }

        QueryMsg::GetAllStocks { limit, start_after } => {
            to_json_binary(&query::get_all_stocks(deps, env, limit, start_after)?)
        }

        QueryMsg::GetStocksByInfluencer {
            influencer,
            limit,
            start_after,
        } => to_json_binary(&query::get_stocks_by_influencer(
            deps,
            env,
            influencer,
            limit,
            start_after,
        )?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::GetStockByIdResponse;
    use crate::state::Stock;
    use crate::utils;
    use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_json};
    use execute::TOTAL_SHARES;

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

    #[test]
    fn test_create_and_query_stock_by_id() {
        let mut deps = mock_dependencies();

        let creator = deps.api.addr_make("creator");
        let msg = InstantiateMsg {};
        let info = message_info(&creator, &coins(1000, "token"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let anyone = deps.api.addr_make("anyone");
        let ticker = "TEST".to_owned();
        let msg = ExecuteMsg::CreateStock {
            ticker: ticker.clone(),
        };
        let info = message_info(&anyone, &coins(100, "token"));

        // we can just call .unwrap() to assert this was a success
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let stock_id: u64 = res
            .attributes
            .iter()
            .find(|attr| attr.key == "stock_id".to_owned())
            .unwrap()
            .value
            //convert from string to u64
            .parse()
            .unwrap();

        // assert that the stock_id starts at 1
        assert_eq!(stock_id, 1);

        // let's get the stock by the stock_id
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetStockById { stock_id },
        )
        .unwrap();

        let value: GetStockByIdResponse = from_json(&res).unwrap();

        assert_eq!(
            GetStockByIdResponse {
                stock: Stock {
                    id: stock_id,
                    ticker,
                    influencer: anyone,
                    total_shares: TOTAL_SHARES,
                    auction_active: false,
                    auction_start: None,
                    auction_end: None,
                    created_at: value.clone().stock.created_at
                }
            },
            value
        );
    }

    #[test]
    fn test_create_all_stocks() {}
}
