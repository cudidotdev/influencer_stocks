use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::{Bid, BuyOrder, Sale, SellOrder, Share, Stock};

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

    CreateBuyOrder {
        stock_id: u64,
        price_per_share: u128,
        shares: u64,
    },

    CreateSellOrder {
        stock_id: u64,
        price_per_share: u128,
        shares: u64,
    },

    CancelBuyOrder {
        buy_order_id: u64,
    },

    CancelSellOrder {
        sell_order_id: u64,
    },

    QuickSell {
        stock_id: u64,
        shares: u64,
        price_per_share: u128,
        // 1 = 1% slippage
        slippage: u64,
    },

    QuickBuy {
        stock_id: u64,
        shares: u64,
        // 1 = 1% slippage
        slippage: u64,
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
        start_after: Option<u64>,
        in_auction: Option<bool>,
        in_sale: Option<bool>,
        marked_as_active_auction: Option<bool>,
    },

    #[returns(GetStocksResponse)]
    GetStocksByInfluencer {
        influencer: Addr,
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

    #[returns(GetSellPriceResponse)]
    GetSellPrice {
        stock_id: u64,
        requested_shares: u64,
    },

    #[returns(GetBuyPriceResponse)]
    GetBuyPrice {
        stock_id: u64,
        requested_shares: u64,
    },

    #[returns(GetBuyOrdersResponse)]
    GetOpenBuyOrdersByStock { stock_id: u64, sort_by: OrderSort },

    #[returns(GetSellPriceResponse)]
    GetOpenBuyOrdersByOwner { owner: Addr, sort_by: OrderSort },

    #[returns(GetBuyOrderByIdResponse)]
    GetBuyOrderById { buy_order_id: u64 },

    #[returns(GetSellPriceResponse)]
    GetOpenSellOrdersByStock { stock_id: u64, sort_by: OrderSort },

    #[returns(GetSellPriceResponse)]
    GetOpenSellOrdersByOwner { owner: Addr, sort_by: OrderSort },

    #[returns(GetSellOrderByIdResponse)]
    GetSellOrderById { sell_order_id: u64 },

    #[returns(GetTotalSellVolumeResponse)]
    GetTotalSellVolume { stock_id: u64 },

    #[returns(GetTotalBuyVolumeResponse)]
    GetTotalBuyVolume { stock_id: u64 },

    #[returns(GetSalesResponse)]
    GetSalesByStock { stock_id: u64 },

    #[returns(GetSaleByIdResponse)]
    GetSalesById { sale_id: u64 },

    #[returns(GetSalesByUserResponse)]
    GetSalesByUser { user: Addr },
}

#[cw_serde]
pub enum OrderSort {
    PriceAsc,
    PriceDesc,
    CreatedAtAsc,
    CreatedAtDesc,
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

#[cw_serde]
pub struct GetSellPriceResponse {
    pub total_price: String,
    pub price_per_share: String,
    pub requested_shares: u64,
}

#[cw_serde]
pub struct GetBuyPriceResponse {
    pub total_price: String,
    pub price_per_share: String,
    pub requested_shares: u64,
}

#[cw_serde]
pub struct GetBuyOrdersResponse {
    pub orders: Vec<BuyOrder>,
}

#[cw_serde]
pub struct GetBuyOrderByIdResponse {
    pub order: BuyOrder,
}

#[cw_serde]
pub struct GetSellOrdersResponse {
    pub orders: Vec<SellOrder>,
}

#[cw_serde]
pub struct GetSellOrderByIdResponse {
    pub order: SellOrder,
}

#[cw_serde]
pub struct GetSalesResponse {
    pub sales: Vec<Sale>,
}

#[cw_serde]
pub struct GetSalesByUserResponse {
    pub buy: Vec<Sale>,
    pub sell: Vec<Sale>,
}

#[cw_serde]
pub struct GetSaleByIdResponse {
    pub sale: Sale,
}

#[cw_serde]
pub struct GetTotalBuyVolumeResponse {
    pub amount: u64,
}

#[cw_serde]
pub struct GetTotalSellVolumeResponse {
    pub amount: u64,
}
