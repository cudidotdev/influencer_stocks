use common::{contract_code, setup_app};
use cosmwasm_std::{coins, Addr};
use cw_multi_test::Executor;
use influencer_stocks::{
    contract::DENOM,
    msg::{
        ExecuteMsg, GetBidsResponse, GetSharesResponse, GetStockByIdResponse, InstantiateMsg,
        QueryMsg,
    },
};

mod common;

#[test]
fn test_end_auction_success() {
    let (mut app, vault) = setup_app();

    // Store the contract code and instantiate
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

    // Create a stock
    let influencer = Addr::unchecked("influencer1");

    let ticker = "INF1_1";
    let create_msg = ExecuteMsg::CreateStock {
        ticker: ticker.to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    // Extract stock_id from response attributes
    let stock_id: u64 = res
        .events
        .iter()
        .flat_map(|event| &event.attributes)
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

    // End auction
    let end_auction_msg = ExecuteMsg::EndAuction { stock_id };
    let res = app
        .execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &end_auction_msg,
            &[],
        )
        .unwrap();

    // Verify response attributes
    let attrs = &res.events.last().unwrap().attributes;
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "end_auction"));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "stock_id" && attr.value == stock_id.to_string()));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "ended_by" && attr.value == influencer.to_string()));
    assert!(attrs.iter().any(|attr| attr.key == "ended_at"));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "is_influencer" && attr.value == "true"));

    // Query stock to verify auction ended
    let query_msg = QueryMsg::GetStockById { stock_id };
    let stock_response: GetStockByIdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    assert!(stock_response.stock.auction_end <= Some(app.block_info().time.nanos() / 1_000_000));
}

#[test]
fn test_place_bids_and_end_auction_success() {
    let (mut app, vault) = setup_app();

    // Store the contract code and instantiate
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

    // Create a stock
    let influencer = app.api().addr_make("influencer");

    let create_msg = ExecuteMsg::CreateStock {
        ticker: "TEST".to_owned(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    // Extract stock_id from response attributes
    let stock_id: u64 = res
        .events
        .iter()
        .flat_map(|event| &event.attributes)
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

    let bidder1 = app.api().addr_make("bidder1");
    let bidder2 = app.api().addr_make("bidder2");

    // fund bidder accounts
    app.send_tokens(vault.clone(), bidder1.clone(), &coins(1_000_000, DENOM))
        .unwrap();

    app.send_tokens(vault.clone(), bidder2.clone(), &coins(950_000 * 11, DENOM))
        .unwrap();

    // Place Bids
    app.execute_contract(
        bidder1.clone(),
        contract_addr.clone(),
        &ExecuteMsg::PlaceBid {
            stock_id,
            price_per_share: 10,
            shares: 100_000,
        },
        &coins(1_000_000, DENOM),
    )
    .unwrap();

    app.execute_contract(
        bidder2.clone(),
        contract_addr.clone(),
        &ExecuteMsg::PlaceBid {
            stock_id,
            price_per_share: 11,
            shares: 950_000,
        },
        &coins(950_000 * 11, DENOM),
    )
    .unwrap();

    // End auction
    let end_auction_msg = ExecuteMsg::EndAuction { stock_id };
    let res = app
        .execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &end_auction_msg,
            &[],
        )
        .unwrap();

    // Verify response attributes
    let attrs = &res.events.last().unwrap().attributes;
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "end_auction"));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "stock_id" && attr.value == stock_id.to_string()));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "ended_by" && attr.value == influencer.to_string()));
    assert!(attrs.iter().any(|attr| attr.key == "ended_at"));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "is_influencer" && attr.value == "true"));

    // Query stock to verify auction ended
    let query_msg = QueryMsg::GetStockById { stock_id };
    let stock_response: GetStockByIdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    assert!(stock_response.stock.auction_end <= Some(app.block_info().time.nanos() / 1_000_000));

    let bids_response: GetBidsResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::GetBidsByStock { stock_id },
        )
        .unwrap();

    // Verfiy that all bids are closed and inactive
    for bid in bids_response.bids {
        assert!(!bid.active);
        assert!(bid.open == 0);
    }

    // Verify that shares were created with proper responses

    let shares_response: GetSharesResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::GetSharesByStock { stock_id },
        )
        .unwrap();

    let shares = shares_response.shares;

    assert_eq!(shares.len(), 2);

    assert_eq!(shares[0].owner, bidder2);
    assert_eq!(shares[0].no_of_shares, 950_000);

    assert_eq!(shares[1].owner, bidder1);
    assert_eq!(shares[1].no_of_shares, 50_000);
}

#[test]
fn test_end_auction_by_owner() {
    let (mut app, vault) = setup_app();

    // Store the contract code and instantiate
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

    // Create a stock
    let influencer = Addr::unchecked("influencer1");

    let ticker = "INF1_1";
    let create_msg = ExecuteMsg::CreateStock {
        ticker: ticker.to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    // Extract stock_id from response attributes
    let stock_id: u64 = res
        .events
        .iter()
        .flat_map(|event| &event.attributes)
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

    // End auction by owner (vault)
    let end_auction_msg = ExecuteMsg::EndAuction { stock_id };
    let res = app
        .execute_contract(vault.clone(), contract_addr.clone(), &end_auction_msg, &[])
        .unwrap();

    // Verify response attributes
    let attrs = &res.events.last().unwrap().attributes;
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "end_auction"));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "stock_id" && attr.value == stock_id.to_string()));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "ended_by" && attr.value == vault.to_string()));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "is_influencer" && attr.value == "false"));

    // Query stock to verify auction ended
    let query_msg = QueryMsg::GetStockById { stock_id };
    let stock_response: GetStockByIdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    assert!(stock_response.stock.auction_end <= Some(app.block_info().time.nanos() / 1_000_000));
}

#[test]
fn test_end_auction_unauthorized() {
    let (mut app, vault) = setup_app();

    // Store the contract code and instantiate
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

    // Create a stock
    let influencer = Addr::unchecked("influencer1");

    let ticker = "INF1_1";
    let create_msg = ExecuteMsg::CreateStock {
        ticker: ticker.to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    // Extract stock_id from response attributes
    let stock_id: u64 = res
        .events
        .iter()
        .flat_map(|event| &event.attributes)
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

    // Try to end auction with unauthorized user
    let random_user = Addr::unchecked("random_user");
    let end_auction_msg = ExecuteMsg::EndAuction { stock_id };

    // This should fail with unauthorized error
    let err = app
        .execute_contract(
            random_user.clone(),
            contract_addr.clone(),
            &end_auction_msg,
            &[],
        )
        .unwrap_err();

    assert!(err.root_cause().to_string().contains("Unauthorized"));
}

#[test]
fn test_end_auction_inactive() {
    let (mut app, vault) = setup_app();

    // Store the contract code and instantiate
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

    // Create a stock
    let influencer = Addr::unchecked("influencer1");

    let ticker = "INF1_1";
    let create_msg = ExecuteMsg::CreateStock {
        ticker: ticker.to_string(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    // Extract stock_id from response attributes
    let stock_id: u64 = res
        .events
        .iter()
        .flat_map(|event| &event.attributes)
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    // Try to end auction when it hasn't started
    let end_auction_msg = ExecuteMsg::EndAuction { stock_id };
    let err = app
        .execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &end_auction_msg,
            &[],
        )
        .unwrap_err();

    assert!(err
        .root_cause()
        .to_string()
        .contains("Stock is yet to be auctioned"));
}

#[test]
fn test_end_auction_nonexistent_stock() {
    let (mut app, vault) = setup_app();

    // Store the contract code and instantiate
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

    // Try to end auction for nonexistent stock
    let influencer = Addr::unchecked("influencer1");
    let nonexistent_stock_id = 999;
    let end_auction_msg = ExecuteMsg::EndAuction {
        stock_id: nonexistent_stock_id,
    };

    let err = app
        .execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &end_auction_msg,
            &[],
        )
        .unwrap_err();

    assert!(err
        .root_cause()
        .to_string()
        .contains(&format!("Stock with id {}", nonexistent_stock_id)));
}
