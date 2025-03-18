use common::{contract_code, setup_app};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use influencer_stocks::msg::{ExecuteMsg, GetStockByIdResponse, InstantiateMsg, QueryMsg};

mod common;

#[test]
fn test_start_auction_success() {
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

    let res = app
        .execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap();

    // Verify response attributes
    let attrs = &res.events.last().unwrap().attributes;
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "start_auction"));
    assert!(attrs
        .iter()
        .any(|attr| attr.key == "stock_id" && attr.value == stock_id.to_string()));
    assert!(attrs.iter().any(|attr| attr.key == "auction_start"));
    assert!(attrs.iter().any(|attr| attr.key == "auction_end"));

    // Query stock to verify auction started
    let query_msg = QueryMsg::GetStockById { stock_id };
    let stock_response: GetStockByIdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    assert!(stock_response.stock.auction_start.is_some());
    assert!(stock_response.stock.auction_end.is_some());
}

#[test]
fn test_start_auction_unauthorized() {
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

    // Try to start auction with different influencer
    let different_influencer = Addr::unchecked("influencer2");

    let start_auction_msg = ExecuteMsg::StartAuction { stock_id };

    // Should fail with unauthorized error
    let err = app
        .execute_contract(
            different_influencer.clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap_err();

    assert!(err.root_cause().to_string().contains("Unauthorized"));
}

#[test]
fn test_start_auction_already_active() {
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

    // Start auction first time
    let start_auction_msg = ExecuteMsg::StartAuction { stock_id };
    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &start_auction_msg,
        &[],
    )
    .unwrap();

    // Try to start auction again
    let err = app
        .execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap_err();

    assert!(err
        .root_cause()
        .to_string()
        .contains("Stock is already been auctioned"));
}

#[test]
fn test_start_auction_nonexistent_stock() {
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

    // Try to start auction for nonexistent stock
    let influencer = Addr::unchecked("influencer1");

    let nonexistent_stock_id = 999;
    let start_auction_msg = ExecuteMsg::StartAuction {
        stock_id: nonexistent_stock_id,
    };

    let err = app
        .execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap_err();

    assert!(err
        .root_cause()
        .to_string()
        .contains(&format!("Stock with id {}", nonexistent_stock_id)));
}

#[test]
fn test_start_auction_after_end() {
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

    // Artificially advance block time to after auction end
    app.update_block(|block| {
        block.time = block.time.plus_seconds(24 * 60 * 60 + 1); // 24 hours + 1 second
    });

    // Try to restart auction
    let err = app
        .execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap_err();

    assert!(err
        .root_cause()
        .to_string()
        .contains("Stock has already been auctioned and in sale"));
}

#[test]
fn test_auction_duration() {
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

    // Query stock to get auction data
    let query_msg = QueryMsg::GetStockById { stock_id };
    let stock_response: GetStockByIdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    let start = stock_response.stock.auction_start.unwrap();
    let end = stock_response.stock.auction_end.unwrap();

    // Verify auction duration is 24 hours (in milliseconds)
    assert_eq!(end - start, 24 * 60 * 60 * 1000);
}
