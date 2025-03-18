use common::{contract_code, setup_app};
use cosmwasm_std::coins;
use cw_multi_test::Executor;
use influencer_stocks::{
    contract::{execute::stocks::TOTAL_SHARES, DENOM},
    msg::{
        ExecuteMsg, GetBidByIdResponse, GetBidsResponse, GetMinimumBidPriceResponse,
        InstantiateMsg, QueryMsg,
    },
    state::Bid,
};

mod common;

#[test]
fn test_place_bid_success() {
    // Setup app with initial balances
    let (mut app, vault) = setup_app();

    // Store and instantiate the contract
    let code_id = app.store_code(contract_code());
    let contract_addr = app
        .instantiate_contract(
            code_id,
            vault.clone(),
            &InstantiateMsg {},
            &[],
            "Influencer Stocks",
            None,
        )
        .unwrap();

    // Create test addresses using valid bech32
    let influencer = app.api().addr_make("influencer");
    let bidder = app.api().addr_make("bidder");
    let bidder_2 = app.api().addr_make("bidder_2");

    // Create a stock
    let create_msg = ExecuteMsg::CreateStock {
        ticker: "TEST".to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    let attrs = res.custom_attrs(1);
    let stock_id: u64 = attrs
        .iter()
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    // Start auction
    let start_auction_msg = ExecuteMsg::StartAuction { stock_id };
    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &start_auction_msg,
        &[],
    )
    .unwrap();

    // Place a bid
    let price_per_share: u128 = 10;
    let shares: u64 = 100_000;
    let total_amount = price_per_share * shares as u128;

    // Verify that min price is 1 uhuahua (0.000001 huahua)
    let query_bid_msg = QueryMsg::GetMinimumBidPrice {
        stock_id,
        shares_requested: shares,
    };

    let res: GetMinimumBidPriceResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_bid_msg)
        .unwrap();

    assert_eq!(res.min_price, "1".to_owned());
    assert_eq!(res.shares_requested, shares);

    // fund bidder account
    app.send_tokens(vault.clone(), bidder.clone(), &coins(total_amount, DENOM))
        .unwrap();

    // get influencer's balance before bid placement
    let influencer_balance_before = app
        .wrap()
        .query_balance(influencer.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    let place_bid_msg = ExecuteMsg::PlaceBid {
        stock_id,
        price_per_share,
        shares,
    };

    let res = app
        .execute_contract(
            bidder.clone(),
            contract_addr.clone(),
            &place_bid_msg,
            &coins(total_amount, DENOM),
        )
        .unwrap();

    // Check response attributes
    let attrs = res.custom_attrs(1);

    assert!(attrs
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "place_bid"));

    let bid_id: u64 = attrs
        .iter()
        .find(|attr| attr.key == "bid_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    // Query the bid to verify
    let query_bid_msg = QueryMsg::GetBidById { bid_id };
    let bid_response: GetBidByIdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_bid_msg)
        .unwrap();

    assert_eq!(bid_response.bid.bidder, bidder);
    assert_eq!(bid_response.bid.stock_id, stock_id);
    assert_eq!(bid_response.bid.price_per_share, price_per_share);
    assert_eq!(bid_response.bid.shares_requested, shares);
    assert_eq!(bid_response.bid.open, 1);
    assert_eq!(bid_response.bid.active, true);

    // Check contract's balance is 0 (all funds disbursed)
    let contract_balance = app
        .wrap()
        .query_balance(contract_addr.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(
        contract_balance, 0,
        "Contract's balance should be 0 after transfers"
    );

    // Verify that the bidders balance is now 0
    let bidder_balance = app
        .wrap()
        .query_balance(bidder.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(bidder_balance, 0);

    // Verify that influencer's balance increased by 1_000_000
    let influencer_balance = app
        .wrap()
        .query_balance(influencer.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(influencer_balance - influencer_balance_before, 1_000_000);

    // Verify bid and share distribution
    let query_bid_msg = QueryMsg::GetOpenBidsByStock { stock_id };

    let res: GetBidsResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_bid_msg)
        .unwrap();

    assert_eq!(
        res,
        GetBidsResponse {
            bids: vec![
                Bid {
                    id: 1,
                    stock_id,
                    bidder: influencer.clone(),
                    price_per_share: 0,
                    shares_requested: 1_000_000,
                    remaining_shares: 900_000,
                    created_at: res.bids[0].created_at,
                    open: 1,
                    active: true
                },
                Bid {
                    id: 2,
                    stock_id,
                    bidder: bidder.clone(),
                    price_per_share: 10,
                    shares_requested: 100_000,
                    remaining_shares: 100_000,
                    created_at: res.bids[1].created_at,
                    open: 1,
                    active: true
                },
            ]
        }
    );

    // PLACING A NEW BID

    let price_per_share: u128 = 11;
    let shares: u64 = 950_000;
    let total_amount = price_per_share * shares as u128;

    // Verify that min price is 11 uhuahua (0.000011 huahua)
    let query_bid_msg = QueryMsg::GetMinimumBidPrice {
        stock_id,
        shares_requested: shares,
    };

    let res: GetMinimumBidPriceResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_bid_msg)
        .unwrap();

    assert_eq!(res.min_price, "11".to_owned());
    assert_eq!(res.shares_requested, shares);

    // fund bidder account
    app.send_tokens(vault.clone(), bidder_2.clone(), &coins(total_amount, DENOM))
        .unwrap();

    // get influencer's balance before bid placement
    let influencer_balance_before = app
        .wrap()
        .query_balance(influencer.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    // Place Bid
    let place_bid_msg = ExecuteMsg::PlaceBid {
        stock_id,
        price_per_share,
        shares,
    };

    app.execute_contract(
        bidder_2.clone(),
        contract_addr.clone(),
        &place_bid_msg,
        &coins(total_amount, DENOM),
    )
    .unwrap();

    // Check contract's balance is 0 (all funds disbursed)
    let contract_balance = app
        .wrap()
        .query_balance(contract_addr.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(
        contract_balance, 0,
        "Contract's balance should be 0 after transfers"
    );

    // Verify that the bidder_2 balance is now 0
    let bidder_2_balance = app
        .wrap()
        .query_balance(bidder_2.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(bidder_2_balance, 0);

    // // Verify that the bidder_1 balance is now 500_000 (refunded bids)
    let bidder_1_balance = app
        .wrap()
        .query_balance(bidder.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(bidder_1_balance, 500_000);

    // Verify that influencer's balance increased by 950_000 * bid_price - refunded bids
    let influencer_balance = app
        .wrap()
        .query_balance(influencer.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(influencer_balance - influencer_balance_before, 9_950_000);

    // Verify bid and share distribution
    let query_bid_msg = QueryMsg::GetOpenBidsByStock { stock_id };

    let res: GetBidsResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_bid_msg)
        .unwrap();

    assert_eq!(
        res,
        GetBidsResponse {
            bids: vec![
                Bid {
                    id: 2,
                    stock_id,
                    bidder: bidder.clone(),
                    price_per_share: 10,
                    shares_requested: 100_000,
                    remaining_shares: 50_000,
                    created_at: res.bids[0].created_at,
                    open: 1,
                    active: true
                },
                Bid {
                    id: 3,
                    stock_id,
                    bidder: bidder_2.clone(),
                    price_per_share: 11,
                    shares_requested: 950_000,
                    remaining_shares: 950_000,
                    created_at: res.bids[1].created_at,
                    open: 1,
                    active: true
                },
            ]
        }
    );

    // Verify that the intial bid is closed

    let query_bid_msg = QueryMsg::GetBidById { bid_id: 1 };
    let bid_response: GetBidByIdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_bid_msg)
        .unwrap();

    assert_eq!(bid_response.bid.open, 0);
    assert_eq!(bid_response.bid.shares_requested, TOTAL_SHARES);
    assert_eq!(bid_response.bid.remaining_shares, 0);
    assert_eq!(bid_response.bid.price_per_share, 0);
    assert_eq!(bid_response.bid.bidder, influencer);
    assert_eq!(bid_response.bid.stock_id, stock_id);
    assert_eq!(bid_response.bid.active, true);
}

#[test]
fn test_auction_inactive() {
    let (mut app, vault) = setup_app();

    let code_id = app.store_code(contract_code());

    let contract_addr = app
        .instantiate_contract(
            code_id,
            vault.clone(),
            &InstantiateMsg {},
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let influencer = app.api().addr_make("influencer");
    let bidder = app.api().addr_make("bidder");

    // Create stock but don't start auction
    let create_msg = ExecuteMsg::CreateStock {
        ticker: "TEST".to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    let stock_id: u64 = res
        .custom_attrs(1)
        .iter()
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    // place bid
    let shares = 100;
    let price_per_share = 10;
    let total_amount = shares as u128 * price_per_share;

    let place_bid_msg = ExecuteMsg::PlaceBid {
        stock_id,
        price_per_share,
        shares,
    };

    // fund bidder account
    app.send_tokens(vault.clone(), bidder.clone(), &coins(total_amount, DENOM))
        .unwrap();

    // Attempt to bid
    let err = app
        .execute_contract(
            bidder.clone(),
            contract_addr.clone(),
            &place_bid_msg,
            &coins(total_amount, DENOM),
        )
        .unwrap_err();

    assert!(err
        .root_cause()
        .to_string()
        .contains("Auction is not active"));
}

#[test]
fn test_excess_fund_refund() {
    let (mut app, vault) = setup_app();

    let code_id = app.store_code(contract_code());

    let contract_addr = app
        .instantiate_contract(
            code_id,
            vault.clone(),
            &InstantiateMsg {},
            &[],
            "Influencer Stocks",
            None,
        )
        .unwrap();

    // Create test addresses
    let influencer = app.api().addr_make("influencer");
    let bidder = app.api().addr_make("bidder");

    // Create and start auction
    let create_msg = ExecuteMsg::CreateStock {
        ticker: "TEST".to_string(),
    };
    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    let stock_id: u64 = res
        .custom_attrs(1)
        .iter()
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::StartAuction { stock_id },
        &[],
    )
    .unwrap();

    // Send 1500 uhuahua for 100 shares @10 (needs 1000)
    app.send_tokens(vault.clone(), bidder.clone(), &coins(1500, DENOM))
        .unwrap();

    app.execute_contract(
        bidder.clone(),
        contract_addr.clone(),
        &ExecuteMsg::PlaceBid {
            stock_id,
            price_per_share: 10,
            shares: 100,
        },
        &coins(1500, DENOM),
    )
    .unwrap();

    // Verify refund
    let bidder_balance = app
        .wrap()
        .query_balance(bidder.to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(bidder_balance, 500); // 1500 sent - 1000 needed
}

#[test]
fn test_bid_on_nonexistent_stock() {
    let (mut app, vault) = setup_app();
    let code_id = app.store_code(contract_code());
    let contract_addr = app
        .instantiate_contract(
            code_id,
            vault.clone(),
            &InstantiateMsg {},
            &[],
            "Influencer Stocks",
            None,
        )
        .unwrap();

    let bidder = app.api().addr_make("bidder");
    let non_existent_stock_id = 999;

    // send funds
    app.send_tokens(vault.clone(), bidder.clone(), &coins(1000, DENOM))
        .unwrap();

    // Attempt to bid on invalid stock
    let err = app
        .execute_contract(
            bidder.clone(),
            contract_addr.clone(),
            &ExecuteMsg::PlaceBid {
                stock_id: non_existent_stock_id,
                price_per_share: 10,
                shares: 100,
            },
            &coins(1000, DENOM),
        )
        .unwrap_err();

    assert!(err
        .root_cause()
        .to_string()
        .contains("Stock with id 999 not found"));
}

#[test]
fn test_bid_on_expired_stock() {
    let (mut app, vault) = setup_app();

    let code_id = app.store_code(contract_code());

    let contract_addr = app
        .instantiate_contract(
            code_id,
            vault.clone(),
            &InstantiateMsg {},
            &[],
            "Influencer Stocks",
            None,
        )
        .unwrap();

    let influencer = app.api().addr_make("influencer");
    let bidder = app.api().addr_make("bidder");

    // Create and start auction
    let create_msg = ExecuteMsg::CreateStock {
        ticker: "TEST".to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    let stock_id: u64 = res
        .custom_attrs(1)
        .iter()
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::StartAuction { stock_id },
        &[],
    )
    .unwrap();

    // Expire auction
    app.update_block(|block| {
        block.time = block.time.plus_seconds(24 * 60 * 60 + 1); // 24h +1s
    });

    // send funds
    app.send_tokens(vault.clone(), bidder.clone(), &coins(1000, DENOM))
        .unwrap();

    // Attempt to bid
    let err = app
        .execute_contract(
            bidder.clone(),
            contract_addr.clone(),
            &ExecuteMsg::PlaceBid {
                stock_id,
                price_per_share: 10,
                shares: 100,
            },
            &coins(1000, DENOM),
        )
        .unwrap_err();

    assert!(err.root_cause().to_string().contains("Auction has ended"));
}

#[test]
fn test_bid_below_minimum_price() {
    let (mut app, vault) = setup_app();

    let code_id = app.store_code(contract_code());

    let contract_addr = app
        .instantiate_contract(
            code_id,
            vault.clone(),
            &InstantiateMsg {},
            &[],
            "Influencer Stocks",
            None,
        )
        .unwrap();

    let influencer = app.api().addr_make("influencer");
    let bidder1 = app.api().addr_make("bidder1");
    let bidder2 = app.api().addr_make("bidder2");

    // Create and start auction
    let create_msg = ExecuteMsg::CreateStock {
        ticker: "TEST".to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    let stock_id: u64 = res
        .custom_attrs(1)
        .iter()
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::StartAuction { stock_id },
        &[],
    )
    .unwrap();

    // place bid
    let shares = TOTAL_SHARES;
    let price_per_share = 10;
    let total_amount = shares as u128 * price_per_share;

    let place_bid_msg = ExecuteMsg::PlaceBid {
        stock_id,
        price_per_share,
        shares,
    };

    // fund bidder1 account
    app.send_tokens(vault.clone(), bidder1.clone(), &coins(total_amount, DENOM))
        .unwrap();

    // Place initial bid
    app.execute_contract(
        bidder1.clone(),
        contract_addr.clone(),
        &place_bid_msg,
        &coins(total_amount, DENOM),
    )
    .unwrap();

    // fund bidder2 account
    app.send_tokens(vault.clone(), bidder2.clone(), &coins(500, DENOM))
        .unwrap();

    // Attempt undercutting bid
    let err = app
        .execute_contract(
            bidder2.clone(),
            contract_addr.clone(),
            &ExecuteMsg::PlaceBid {
                stock_id,
                price_per_share: 10, // Should be at least 11
                shares: 50,
            },
            &coins(500, DENOM),
        )
        .unwrap_err();

    assert!(err.root_cause().to_string().contains("Bid price too low"));
}

#[test]
fn test_sent_funds_less_than_expected_amount() {
    let (mut app, vault) = setup_app();

    let code_id = app.store_code(contract_code());

    let contract_addr = app
        .instantiate_contract(
            code_id,
            vault.clone(),
            &InstantiateMsg {},
            &[],
            "Influencer Stocks",
            None,
        )
        .unwrap();

    let influencer = app.api().addr_make("influencer");
    let bidder = app.api().addr_make("bidder1");

    // Create and start auction
    let create_msg = ExecuteMsg::CreateStock {
        ticker: "TEST".to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    let stock_id: u64 = res
        .custom_attrs(1)
        .iter()
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::StartAuction { stock_id },
        &[],
    )
    .unwrap();

    // place bid
    let shares = 100;
    let price_per_share = 10;
    let total_amount = shares as u128 * price_per_share;

    let place_bid_msg = ExecuteMsg::PlaceBid {
        stock_id,
        price_per_share,
        shares,
    };

    // fund bidder1 account
    app.send_tokens(vault.clone(), bidder.clone(), &coins(total_amount, DENOM))
        .unwrap();

    // Place bid
    let err = app
        .execute_contract(
            bidder.clone(),
            contract_addr.clone(),
            &place_bid_msg,
            &coins(total_amount - 100, DENOM),
        )
        .unwrap_err();

    assert!(err.root_cause().to_string().contains(&format!(
        "Insufficient funds sent. Expected {total_amount}, got {}",
        total_amount - 100
    )));
}
