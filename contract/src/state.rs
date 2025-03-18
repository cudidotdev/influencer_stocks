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
    pub created_at: u64,
}

// Index for Stocks
pub struct StockIndexes<'a> {
    // Secondary indexes
    pub influencer: MultiIndex<'a, Addr, Stock, &'a [u8]>,
}

impl IndexList<Stock> for StockIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Stock>> + '_> {
        let v = vec![&self.influencer as &dyn Index<Stock>];
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
    pub remaining_shares: u64, // Number of shares that haven't been outbid
    pub created_at: u64,
    pub open: u8, // Whether the bid is has still open (has not been outbid)
    pub active: bool, // Whether the stock is still in auction
                  // (all inactive bids are closed)
}

// Index for Bids
pub struct BidIndexes<'a> {
    pub stock_id: MultiIndex<'a, u64, Bid, &'a [u8]>,
    pub bidder: MultiIndex<'a, Addr, Bid, &'a [u8]>,
    pub stock_open: MultiIndex<'a, (u64, u8), Bid, &'a [u8]>,
}

impl IndexList<Bid> for BidIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Bid>> + '_> {
        let v = vec![
            &self.stock_id as &dyn Index<Bid>,
            &self.bidder as &dyn Index<Bid>,
            &self.stock_open as &dyn Index<Bid>,
        ];
        Box::new(v.into_iter())
    }
}

// Create indexes
pub const BID_INDEXES: BidIndexes = BidIndexes {
    stock_id: MultiIndex::new(|_pk, bid| bid.stock_id, "bids", "bids__stock_id"),
    bidder: MultiIndex::new(|_pk, bid| bid.bidder.clone(), "bids", "bids__bidder"),
    stock_open: MultiIndex::new(
        |_pk, bid| (bid.stock_id, bid.open),
        "bids",
        "bids__stock_open",
    ),
};

pub const BIDS: IndexedMap<&[u8], Bid, BidIndexes> = IndexedMap::new("bids", BID_INDEXES);
pub const BID_COUNT: Item<u64> = Item::new("bid_count");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Share {
    pub id: u64,
    pub stock_id: u64,
    pub no_of_shares: u64,
    pub owner: Addr,
}

// Index for Stakes
pub struct ShareIndexes<'a> {
    pub stock_id: MultiIndex<'a, u64, Share, &'a [u8]>,
    pub owner: MultiIndex<'a, Addr, Share, &'a [u8]>,
}

impl IndexList<Share> for ShareIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Share>> + '_> {
        let v = vec![
            &self.stock_id as &dyn Index<Share>,
            &self.owner as &dyn Index<Share>,
        ];
        Box::new(v.into_iter())
    }
}

// Create indexes
pub const SHARE_INDEXES: ShareIndexes = ShareIndexes {
    stock_id: MultiIndex::new(|_pk, share| share.stock_id, "share", "share__stock_id"),
    owner: MultiIndex::new(|_pk, share| share.owner.clone(), "share", "share__owner"),
};

pub const SHARES: IndexedMap<&[u8], Share, ShareIndexes> = IndexedMap::new("share", SHARE_INDEXES);
pub const SHARE_COUNT: Item<u64> = Item::new("share_count");
