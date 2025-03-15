use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::Stock;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateStock { ticker: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetStockByIdResponse)]
    GetStockById { stock_id: u64 },

    #[returns(GetStocksResponse)]
    GetAllStocks {
        limit: Option<usize>,
        start_after: Option<u64>,
    },

    #[returns(GetStocksResponse)]
    GetStocksByInfluencer {
        influencer: Addr,
        limit: Option<usize>,
        start_after: Option<u64>,
    },
}

#[cw_serde]
pub struct GetStockByIdResponse {
    pub stock: Stock,
}

#[cw_serde]
pub struct GetStocksResponse {
    pub stocks: Vec<Stock>,
}
