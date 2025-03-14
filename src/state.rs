use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

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

// Storage items
pub const STATE: Item<State> = Item::new("state");
pub const STOCKS: Map<&[u8], Stock> = Map::new("stocks");
pub const STOCK_COUNT: Item<u64> = Item::new("stock_count");
