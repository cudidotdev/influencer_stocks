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
fn test_get_stocks_by_auction_status() {
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
    let mut stock_ids = Vec::new();

    for i in 1..=num_stocks {
        let influencer = Addr::unchecked(format!("influencer{}", i));
        influencers.push(influencer.clone());

        // Create stock
        let ticker = format!("INFL{}", i);
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

    // Start auction for the first 3 stocks
    for i in 0..3 {
        let start_auction_msg = ExecuteMsg::StartAuction {
            stock_id: stock_ids[i],
        };
        app.execute_contract(
            influencers[i].clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap();
    }

    // Query stocks in auction
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: Some(true),
        in_sale: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify that only the first 3 stocks are in auction
    assert_eq!(response.stocks.len(), 3);

    // The stocks should be in descending order by ID
    for (i, stock) in response.stocks.iter().enumerate() {
        assert_eq!(stock.id, stock_ids[2 - i]);
        assert!(stock.auction_start.is_some());
        assert!(stock.auction_end > Some(app.block_info().time.nanos() / 1_000_000));
    }

    // Query stocks not in auction
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: Some(false),
        in_sale: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify that 2 stocks are not in auction
    assert_eq!(response.stocks.len(), 2);

    // The stocks should be in descending order by ID
    for (i, stock) in response.stocks.iter().enumerate() {
        assert_eq!(stock.id, stock_ids[4 - i]);
        assert!(stock.auction_start.is_none());
    }
}

#[test]
fn test_get_stocks_by_sale_status() {
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
    let mut stock_ids = Vec::new();

    for i in 1..=num_stocks {
        let influencer = Addr::unchecked(format!("influencer{}", i));
        influencers.push(influencer.clone());

        // Create stock
        let ticker = format!("INFL{}", i);
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

    // Start and end auction for the first 3 stocks
    for i in 0..3 {
        // Start auction
        let start_auction_msg = ExecuteMsg::StartAuction {
            stock_id: stock_ids[i],
        };
        app.execute_contract(
            influencers[i].clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap();

        // End auction
        let end_auction_msg = ExecuteMsg::EndAuction {
            stock_id: stock_ids[i],
        };
        app.execute_contract(
            influencers[i].clone(),
            contract_addr.clone(),
            &end_auction_msg,
            &[],
        )
        .unwrap();
    }

    // Query stocks in sale (auction ended)
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: None,
        in_sale: Some(true),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify that only the first 3 stocks are in sale
    assert_eq!(response.stocks.len(), 3);

    // The stocks should be in descending order by ID
    for (i, stock) in response.stocks.iter().enumerate() {
        assert_eq!(stock.id, stock_ids[2 - i]);
        assert!(stock.auction_start.is_some());
        assert!(stock.auction_end <= Some(app.block_info().time.nanos() / 1_000_000));
    }

    // Query stocks not in sale
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: None,
        in_sale: Some(false),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify that 2 stocks are not in sale
    assert_eq!(response.stocks.len(), 2);

    // The stocks should be in descending order by ID
    for (i, stock) in response.stocks.iter().enumerate() {
        assert_eq!(stock.id, stock_ids[4 - i]);
        assert!(stock.auction_start.is_none());
    }
}

#[test]
fn test_get_stocks_with_combined_filters() {
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

    // Create 6 stocks with different influencers
    let num_stocks = 6;
    let mut influencers = Vec::new();
    let mut stock_ids = Vec::new();

    for i in 1..=num_stocks {
        let influencer = Addr::unchecked(format!("influencer{}", i));
        influencers.push(influencer.clone());

        // Create stock
        let ticker = format!("INFL{}", i);
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

    // Status setup:
    // Stock 0, 1: Start auction, end auction (in_sale = true, in_auction = false)
    // Stock 2, 3: Start auction only (in_sale = false, in_auction = true)
    // Stock 4, 5: No auction (in_sale = false, in_auction = false)

    // Start and end auction for stocks 0 and 1
    for i in 0..2 {
        // Start auction
        let start_auction_msg = ExecuteMsg::StartAuction {
            stock_id: stock_ids[i],
        };
        app.execute_contract(
            influencers[i].clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap();

        // End auction
        let end_auction_msg = ExecuteMsg::EndAuction {
            stock_id: stock_ids[i],
        };
        app.execute_contract(
            influencers[i].clone(),
            contract_addr.clone(),
            &end_auction_msg,
            &[],
        )
        .unwrap();
    }

    // Start auction for stocks 2 and 3
    for i in 2..4 {
        let start_auction_msg = ExecuteMsg::StartAuction {
            stock_id: stock_ids[i],
        };
        app.execute_contract(
            influencers[i].clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap();
    }

    // Test combined filters - in_auction=true, in_sale=false
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: Some(true),
        in_sale: Some(false),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Should return stocks 2 and 3 (in auction but not in sale)
    assert_eq!(response.stocks.len(), 2);
    for (i, stock) in response.stocks.iter().enumerate() {
        assert_eq!(stock.id, stock_ids[3 - i]);
        assert!(stock.auction_start.is_some());
        assert!(stock.auction_end > Some(app.block_info().time.nanos() / 1_000_000));
    }

    // Test combined filters - in_auction=false, in_sale=true
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: Some(false),
        in_sale: Some(true),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Should return stocks 0 and 1 (not in auction but in sale)
    assert_eq!(response.stocks.len(), 2);
    for (i, stock) in response.stocks.iter().enumerate() {
        assert_eq!(stock.id, stock_ids[1 - i]);
        assert!(stock.auction_start.is_some());
        assert!(stock.auction_end <= Some(app.block_info().time.nanos() / 1_000_000));
    }

    // Test combined filters - in_auction=false, in_sale=false
    let query_msg = QueryMsg::GetAllStocks {
        start_after: None,
        in_auction: Some(false),
        in_sale: Some(false),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Should return stocks 4 and 5 (not in auction and not in sale)
    assert_eq!(response.stocks.len(), 2);
    for (i, stock) in response.stocks.iter().enumerate() {
        assert_eq!(stock.id, stock_ids[5 - i]);
        assert!(stock.auction_start.is_none());
    }

    // Test with start_after combined with filters
    let query_msg = QueryMsg::GetAllStocks {
        start_after: Some(stock_ids[3]),
        in_auction: Some(true),
        in_sale: Some(false),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Should return stock 2 only (in auction but not in sale, after stock 3)
    assert_eq!(response.stocks.len(), 1);
    assert_eq!(response.stocks[0].id, stock_ids[2]);
}
