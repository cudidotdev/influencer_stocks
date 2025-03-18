use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::{Bid, Share, Stock};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateStock {
        ticker: String,
    },

    StartAuction {
        stock_id: u64,
    },

    EndAuction {
        stock_id: u64,
    },

    PlaceBid {
        stock_id: u64,
        price_per_share: u128,
        shares: u64,
    },
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

    #[returns(GetStocksResponse)]
    GetActiveAuctions {
        limit: Option<usize>,
        start_after: Option<u64>,
    },

    #[returns(GetStocksResponse)]
    GetExpiredActiveAuctions {
        limit: Option<usize>,
        start_after: Option<u64>,
    },

    #[returns(GetBidsResponse)]
    GetBidsByBidder {
        bidder: Addr,
        is_open: Option<bool>,
        is_active: Option<bool>,
        stock_id: Option<u64>,
    },

    #[returns(GetBidsResponse)]
    GetOpenBidsByStock { stock_id: u64 },

    #[returns(GetBidsResponse)]
    GetBidsByStock { stock_id: u64 },

    #[returns(GetBidByIdResponse)]
    GetBidById { bid_id: u64 },

    #[returns(GetMinimumBidPriceResponse)]
    GetMinimumBidPrice {
        stock_id: u64,
        shares_requested: u64,
    },

    #[returns(GetSharesResponse)]
    GetSharesByStock { stock_id: u64 },

    #[returns(GetSharesResponse)]
    GetSharesByOwner { owner: Addr },

    #[returns(GetShareByIdResponse)]
    GetShareById { share_id: u64 },
}

#[cw_serde]
pub struct GetStockByIdResponse {
    pub stock: Stock,
}

#[cw_serde]
pub struct GetStocksResponse {
    pub stocks: Vec<Stock>,
}

#[cw_serde]
pub struct GetBidsResponse {
    pub bids: Vec<Bid>,
}

#[cw_serde]
pub struct GetBidByIdResponse {
    pub bid: Bid,
}

#[cw_serde]
pub struct GetMinimumBidPriceResponse {
    pub min_price: String,
    pub shares_requested: u64,
}

#[cw_serde]
pub struct GetShareByIdResponse {
    pub share: Share,
}

#[cw_serde]
pub struct GetSharesResponse {
    pub shares: Vec<Share>,
}
