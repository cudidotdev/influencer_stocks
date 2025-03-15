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
    pub auction_active: bool,
    pub auction_start: Option<i64>,
    pub auction_end: Option<i64>,
    pub created_at: i64,
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
