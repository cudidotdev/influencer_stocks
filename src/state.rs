use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Stock {
    pub id: u64,
    pub ticker: String,
    pub influencer: Addr,
    pub total_shares: u64,
    pub auction_start: Option<u64>,
    pub auction_end: Option<u64>,
    pub auction_active: u8,
    pub created_at: u64,
}

// Index for Stocks
pub struct StockIndexes<'a> {
    // Secondary indexes
    pub influencer: MultiIndex<'a, Addr, Stock, &'a [u8]>,
    pub auction_active: MultiIndex<'a, u8, Stock, &'a [u8]>,
}

impl IndexList<Stock> for StockIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Stock>> + '_> {
        let v = vec![
            &self.influencer as &dyn Index<Stock>,
            &self.auction_active as &dyn Index<Stock>,
        ];
        Box::new(v.into_iter())
    }
}

// Create indexes
pub const STOCK_INDEXES: StockIndexes = StockIndexes {
    influencer: MultiIndex::new(
        |_pk, stock| stock.influencer.clone(),
        "stocks",
        "stocks__influencer",
    ),
    auction_active: MultiIndex::new(
        |_pk, stock| stock.auction_active,
        "stocks",
        "stocks__auction_end",
    ),
};

pub const STOCKS: IndexedMap<&[u8], Stock, StockIndexes> = IndexedMap::new("stocks", STOCK_INDEXES);
pub const STOCK_COUNT: Item<u64> = Item::new("stock_count");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Bid {
    pub id: u64,
    pub stock_id: u64,
    pub bidder: Addr,
    pub price_per_share: u128, // Price in smallest unit (e.g., uhuahua)
    pub shares_requested: u64,
    pub outbid_shares: u64, // Number of shares that were outbid
    pub total_amount: u128, // Total amount bid (price * shares)
    pub created_at: u64,
    pub updated_at: u64,
    pub outbid: bool, // Whether the bid is has been outbidded
}

// Index for Bids
pub struct BidIndexes<'a> {
    pub stock_id: MultiIndex<'a, u64, Bid, &'a [u8]>,
    pub bidder: MultiIndex<'a, Addr, Bid, &'a [u8]>,
    pub price: MultiIndex<'a, u128, Bid, &'a [u8]>,
}

impl IndexList<Bid> for BidIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Bid>> + '_> {
        let v = vec![
            &self.stock_id as &dyn Index<Bid>,
            &self.bidder as &dyn Index<Bid>,
            &self.price as &dyn Index<Bid>,
        ];
        Box::new(v.into_iter())
    }
}

// Create indexes
pub const BID_INDEXES: BidIndexes = BidIndexes {
    stock_id: MultiIndex::new(|_pk, bid| bid.stock_id, "bids", "bids__stock_id"),
    bidder: MultiIndex::new(|_pk, bid| bid.bidder.clone(), "bids", "bids__bidder"),
    price: MultiIndex::new(|_pk, bid| bid.price_per_share, "bids", "bids__price"),
};

pub const BIDS: IndexedMap<&[u8], Bid, BidIndexes> = IndexedMap::new("bids", BID_INDEXES);
pub const BID_COUNT: Item<u64> = Item::new("bid_count");
