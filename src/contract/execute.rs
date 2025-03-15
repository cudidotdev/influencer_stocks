#![allow(unused_variables)]

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{Stock, STOCKS, STOCK_COUNT},
    ContractError,
};

use chrono::Utc;

// total shares for a stock fixed at 1_000_000 for now,
// may be updated to vary based on creators choice
pub const TOTAL_SHARES: u64 = 1_000_0000;

pub fn create_stock(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ticker: String,
) -> Result<Response, ContractError> {
    // Validate addresses
    let influencer_addr = info.sender.clone();

    // increment and save stock_id counter
    let stock_id = STOCK_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
    STOCK_COUNT.save(deps.storage, &stock_id)?;

    let created_at = Utc::now().timestamp_millis();

    // Create new stock
    let stock = Stock {
        id: stock_id,
        ticker,
        influencer: influencer_addr,
        total_shares: TOTAL_SHARES,
        auction_active: false,
        auction_start: None,
        auction_end: None,
        created_at,
    };

    // Save the stock
    STOCKS.save(deps.storage, stock_id.to_be_bytes().as_slice(), &stock)?;

    Ok(Response::new()
        .add_attribute("action", "create_stock")
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("influencer", info.sender)
        .add_attribute("created_at", created_at.to_string())
        .add_attribute("total_shares", TOTAL_SHARES.to_string()))
}
