use common::{contract_code, setup_app};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use influencer_stocks::msg::{ExecuteMsg, GetStockByIdResponse, InstantiateMsg, QueryMsg};

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

    assert_eq!(stock_response.stock.auction_active, 0);
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

    assert_eq!(stock_response.stock.auction_active, 0);
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
        .contains("Auction is not active"));
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
