use cosmwasm_std::{
    coins, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};

use crate::{
    state::{Bid, BIDS, BID_COUNT, STATE, STOCKS},
    ContractError,
};

use format as f;

// Minimum bid increment (0.000001 huahua)
const MIN_BID_INCREMENT: u128 = 1;

// Denomination of the token we're using
const DENOM: &str = "uhuahua";

pub fn place_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stock_id: u64,
    price_per_share: u128,
    shares: u64,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    // Check if funds are sent
    let sent_funds = info.funds.iter().find(|coin| coin.denom == DENOM);
    if sent_funds.is_none() {
        return Err(ContractError::GenericError(f!("No {DENOM} sent")));
    }

    let sent_amount = sent_funds.unwrap().amount.u128();
    let expected_amount = (price_per_share * shares as u128);

    // Validate funds
    if sent_amount < expected_amount {
        return Err(ContractError::GenericError(f!(
            "Insufficient funds sent. Expected {expected_amount}, got {sent_amount}"
        )));
    }

    // Load the stock
    let stock_id_bytes = stock_id.to_be_bytes();
    let stock = STOCKS
        .load(deps.storage, &stock_id_bytes)
        .map_err(|_| ContractError::NotFound(f!("Stock with id {stock_id}")))?;

    // Check if auction is active
    if stock.auction_active != 1 {
        return Err(ContractError::GenericError(f!("Auction is not active")));
    }

    // Check if auction has expired
    let current_time = env.block.time.nanos() / 1_000_000;
    if let Some(end_timestamp) = stock.auction_end {
        if current_time > end_timestamp {
            return Err(ContractError::GenericError(f!("Auction has ended")));
        }
    }

    // Check if the user already has a bid for this stock
    let existing_bids = BIDS
        .idx
        .bidder
        .prefix(info.sender.clone())
        .range(deps.storage, None, None, Order::Ascending)
        .filter(|item| match item {
            Ok((_, bid)) => bid.stock_id == stock_id,
            _ => false,
        })
        .collect::<StdResult<Vec<_>>>()?;

    if !existing_bids.is_empty() {
        // Instead of placing a new bid, update the existing one
        return update_existing_bid(
            deps,
            env,
            info,
            existing_bids[0].1.id,
            price_per_share,
            shares,
            sent_amount,
        );
    }

    // Get the current minimum bid price
    let min_bid_price = get_minimum_bid_price(deps.as_ref(), stock_id, shares)?;

    // Check if the bid price is greater than or equal to the minimum bid price
    if price_per_share < min_bid_price {
        return Err(ContractError::GenericError(f!(
            "Bid price too low. Minimum price is {min_bid_price}"
        )));
    }

    // Create a new bid
    let bid_id = BID_COUNT.may_load(deps.storage)?.unwrap_or(0) + 1;
    BID_COUNT.save(deps.storage, &bid_id)?;

    let bid = Bid {
        id: bid_id,
        stock_id,
        bidder: info.sender.clone(),
        price_per_share,
        shares_requested: shares,
        outbid_shares: 0,
        total_amount: price_per_share * shares as u128,
        created_at: current_time,
        updated_at: current_time,
        outbid: false,
    };

    // Save the bid
    BIDS.save(deps.storage, &bid_id.to_be_bytes(), &bid)?;

    // Process outbidding
    let outbid_result = process_outbidding(deps, env, stock_id, bid_id)?;

    Ok(Response::new()
        .add_attribute("action", "place_bid")
        .add_attribute("bid_id", bid_id.to_string())
        .add_attribute("stock_id", stock_id.to_string())
        .add_attribute("bidder", info.sender.to_string())
        .add_attribute("price_per_share", price_per_share.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute(
            "total_amount",
            (price_per_share * shares as u128).to_string(),
        )
        .add_attribute("outbid_bids", outbid_result.to_string())
        .add_messages(vec![
            // Transfer the funds to contract owner (vault)
            CosmosMsg::Bank(BankMsg::Send {
                to_address: state.owner.to_string(),
                amount: coins(sent_amount, DENOM),
            }),
        ]))
}
