use common::{contract_code, setup_app, DENOM};
use cosmwasm_std::{coins, Addr};
use cw_multi_test::Executor;
use influencer_stocks::msg::{ExecuteMsg, GetStocksResponse, InstantiateMsg, QueryMsg};

mod common;

#[test]
fn test_get_stocks_by_influencer() {
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

    // Create 3 stocks for influencer1
    let influencer1 = Addr::unchecked("influencer1");
    app.send_tokens(vault.clone(), influencer1.clone(), &coins(300, DENOM))
        .unwrap();

    for i in 1..=3 {
        let ticker = format!("INF1_{}", i);
        let create_msg = ExecuteMsg::CreateStock {
            ticker: ticker.clone(),
        };

        app.execute_contract(influencer1.clone(), contract_addr.clone(), &create_msg, &[])
            .unwrap();
    }

    // Create 2 stocks for influencer2
    let influencer2 = Addr::unchecked("influencer2");
    app.send_tokens(vault.clone(), influencer2.clone(), &coins(200, DENOM))
        .unwrap();

    for i in 1..=2 {
        let ticker = format!("INF2_{}", i);
        let create_msg = ExecuteMsg::CreateStock {
            ticker: ticker.clone(),
        };

        app.execute_contract(influencer2.clone(), contract_addr.clone(), &create_msg, &[])
            .unwrap();
    }

    // Query stocks for influencer1
    let query_msg = QueryMsg::GetStocksByInfluencer {
        influencer: influencer1.clone(),
        limit: None,
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify only influencer1's stocks are returned (3)
    assert_eq!(response.stocks.len(), 3);

    // Verify all stocks belong to influencer1
    for stock in &response.stocks {
        assert_eq!(stock.influencer, influencer1);
        assert!(stock.ticker.starts_with("INF1_"));
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }

    // Query stocks for influencer2
    let query_msg = QueryMsg::GetStocksByInfluencer {
        influencer: influencer2.clone(),
        limit: None,
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify only influencer2's stocks are returned (2)
    assert_eq!(response.stocks.len(), 2);

    // Verify all stocks belong to influencer2
    for stock in &response.stocks {
        assert_eq!(stock.influencer, influencer2);
        assert!(stock.ticker.starts_with("INF2_"));
    }

    // Verify total stocks across all influencers
    let query_msg = QueryMsg::GetAllStocks {
        limit: None,
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Total should be sum of all influencer stock counts (3+2=6)
    assert_eq!(response.stocks.len(), 5);
}

#[test]
fn test_get_stocks_by_influencer_with_limit() {
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

    // Create 5 stocks for the same influencer
    let influencer = Addr::unchecked("influencer1");
    app.send_tokens(vault.clone(), influencer.clone(), &coins(500, DENOM))
        .unwrap();

    for i in 1..=5 {
        let ticker = format!("INF1_{}", i);
        let create_msg = ExecuteMsg::CreateStock {
            ticker: ticker.clone(),
        };

        app.execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
            .unwrap();
    }

    // Query with limit
    let limit = 3;
    let query_msg = QueryMsg::GetStocksByInfluencer {
        influencer: influencer.clone(),
        limit: Some(limit),
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify number of stocks respects limit
    assert_eq!(response.stocks.len(), limit);

    // Verify all stocks belong to the influencer
    for stock in &response.stocks {
        assert_eq!(stock.influencer, influencer);
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }
}

#[test]
fn test_get_stocks_by_influencer_with_start_after() {
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

    // Create 5 stocks for the same influencer
    let influencer = Addr::unchecked("influencer1");
    app.send_tokens(vault.clone(), influencer.clone(), &coins(500, DENOM))
        .unwrap();

    let mut stock_ids = Vec::new();

    for i in 1..=5 {
        let ticker = format!("INF1_{}", i);
        let create_msg = ExecuteMsg::CreateStock {
            ticker: ticker.clone(),
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

        stock_ids.push(stock_id);
    }

    // Get ID of the 3rd stock for start_after parameter
    let start_after = stock_ids[2];

    let query_msg = QueryMsg::GetStocksByInfluencer {
        influencer: influencer.clone(),
        limit: None,
        start_after: Some(start_after),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify we get only stocks with IDs less than start_after
    assert_eq!(response.stocks.len(), 2);

    // Verify all stocks belong to the influencer and have ID less than start_after
    for stock in &response.stocks {
        assert_eq!(stock.influencer, influencer);
        assert!(stock.id < start_after);
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }
}
