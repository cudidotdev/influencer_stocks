use common::{contract_code, setup_app};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use influencer_stocks::msg::{
    ExecuteMsg, GetStockByIdResponse, GetStocksResponse, InstantiateMsg, QueryMsg,
};

mod common;

#[test]
fn test_query_non_existent_stock() {
    let (mut app, vault) = setup_app();

    let code_id = app.store_code(contract_code());

    // Instantiate the contract
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

    // Query a non-existent stock
    let query_msg = QueryMsg::GetStockById { stock_id: 999 };

    // Verify query fails
    let result: Result<GetStockByIdResponse, _> = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg);

    // Error Message should contain `stock with id 999 not found`
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("Stock with id 999 not found"));
}

#[test]
fn test_get_all_stocks_empty() {
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

    // Query all stocks when none exist
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: None,
        in_sale: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify empty list
    assert_eq!(response.stocks.len(), 0);
}

#[test]
fn test_get_all_stocks() {
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

    // Create 5 stocks with different influencers
    let num_stocks = 5;
    let mut influencers = Vec::new();

    for i in 1..=num_stocks {
        let influencer = Addr::unchecked(format!("influencer{}", i));
        influencers.push(influencer.clone());

        // Create stock
        let ticker = format!("INFL{}", i);
        let create_msg = ExecuteMsg::CreateStock {
            ticker: ticker.clone(),
        };

        app.execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
            .unwrap();
    }

    // Query all stocks
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: None,
        in_sale: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify number of stocks
    assert_eq!(response.stocks.len(), num_stocks as usize);

    // Verify stocks are in descending order by ID
    for (i, stock) in response.stocks.iter().enumerate() {
        let expected_id = num_stocks - i as u64;
        assert_eq!(stock.id, expected_id);
        assert_eq!(stock.ticker, format!("INFL{}", expected_id));
        assert_eq!(stock.influencer, influencers[expected_id as usize - 1]);
    }
}

#[test]
fn test_get_all_stocks_with_start_after() {
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

    // Create 10 stocks
    let num_stocks = 10;

    for i in 1..=num_stocks {
        let influencer = Addr::unchecked(format!("influencer{}", i));

        // Create stock
        let ticker = format!("INFL{}", i);
        let create_msg = ExecuteMsg::CreateStock {
            ticker: ticker.clone(),
        };

        app.execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
            .unwrap();
    }

    // Query with start_after
    let start_after = 7; // Skip stocks with IDs 10, 9, 8, 7
    let query_msg = QueryMsg::GetAllStocks {
        start_after: Some(start_after),
        in_auction: None,
        in_sale: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify correct number of stocks returned (IDs 6, 5, 4, 3, 2, 1)
    assert_eq!(response.stocks.len(), (start_after - 1) as usize);

    // Verify stocks are in descending order by ID
    for (i, stock) in response.stocks.iter().enumerate() {
        let expected_id = 6 - i as u64;
        assert_eq!(stock.id, expected_id);
        assert_eq!(stock.ticker, format!("INFL{}", expected_id));
    }
}

#[test]
fn test_get_stocks_by_influencer_empty() {
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

    // Query stocks for a non-existent influencer
    let influencer = Addr::unchecked("non_existent_influencer");
    let query_msg = QueryMsg::GetStocksByInfluencer {
        influencer,
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify empty list
    assert_eq!(response.stocks.len(), 0);
}
