use common::{contract_code, setup_app};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use influencer_stocks::msg::{ExecuteMsg, GetStocksResponse, InstantiateMsg, QueryMsg};

mod common;

#[test]
fn test_get_active_auctions_empty() {
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

    // Query active auctions when none exist
    let query_msg = QueryMsg::GetActiveAuctions {
        limit: None,
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify empty list
    assert_eq!(response.stocks.len(), 0);
}

#[test]
fn test_get_active_auctions() {
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
    let mut stock_ids = Vec::new();

    for i in 1..=num_stocks {
        let influencer = Addr::unchecked(format!("influencer{}", i));

        // Create stock
        let ticker = format!("INFL{}", i);
        let create_msg = ExecuteMsg::CreateStock {
            ticker: ticker.clone(),
        };

        let res = app
            .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
            .unwrap();

        // Extract stock_id
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

        // Start auction for odd-numbered stocks only
        if i % 2 == 1 {
            let start_auction_msg = ExecuteMsg::StartAuction { stock_id };
            app.execute_contract(
                influencer.clone(),
                contract_addr.clone(),
                &start_auction_msg,
                &[],
            )
            .unwrap();
        }
    }

    // Query active auctions
    let query_msg = QueryMsg::GetActiveAuctions {
        limit: None,
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify number of active auctions (stocks 1, 3, 5)
    assert_eq!(response.stocks.len(), 3);

    // Verify all returned stocks have auction_active = 1
    for stock in &response.stocks {
        assert_eq!(stock.auction_active, 1);
        // Verify only odd-numbered stocks are in the result
        assert_eq!(stock.id % 2, 1);
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }
}

#[test]
fn test_get_active_auctions_with_limit() {
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

    // Create 5 stocks with the same influencer
    let influencer = Addr::unchecked("influencer1");
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

    // Start auctions for all 5 stocks
    for &stock_id in &stock_ids {
        let start_auction_msg = ExecuteMsg::StartAuction { stock_id };
        app.execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap();
    }

    // Query with limit
    let limit = 3;
    let query_msg = QueryMsg::GetActiveAuctions {
        limit: Some(limit),
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify number of stocks respects limit
    assert_eq!(response.stocks.len(), limit);

    // Verify all returned stocks have active auctions
    for stock in &response.stocks {
        assert_eq!(stock.auction_active, 1);
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }
}

#[test]
fn test_get_active_auctions_with_start_after() {
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

    // Create 5 stocks with the same influencer
    let influencer = Addr::unchecked("influencer1");
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

    // Start auctions for all 5 stocks
    for &stock_id in &stock_ids {
        let start_auction_msg = ExecuteMsg::StartAuction { stock_id };
        app.execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap();
    }

    // Get ID of the 3rd stock for start_after parameter
    let start_after = stock_ids[2];

    let query_msg = QueryMsg::GetActiveAuctions {
        limit: None,
        start_after: Some(start_after),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify we get only stocks with IDs less than start_after
    assert_eq!(response.stocks.len(), 2);

    // Verify all returned stocks have ID less than start_after
    for stock in &response.stocks {
        assert!(stock.id < start_after);
        assert_eq!(stock.auction_active, 1);
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }
}
