use common::{contract_code, setup_app};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use influencer_stocks::msg::{
    ExecuteMsg, GetStockByIdResponse, GetStocksResponse, InstantiateMsg, QueryMsg,
};

mod common;

#[test]
fn test_get_expired_active_auctions() {
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

    // Create 5 stocks
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

    // Verify no expired auctions initially
    let query_msg = QueryMsg::GetExpiredActiveAuctions {
        limit: None,
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    assert_eq!(response.stocks.len(), 0);

    // Advance block time to expire all 5 auctions
    app.update_block(|block| {
        block.time = block.time.plus_seconds(24 * 60 * 60 + 1); // 24 hours + 1 second
    });

    // Query expired active auctions
    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify all 5 auctions are now expired
    assert_eq!(response.stocks.len(), 5);

    // End 2 auctions manually
    for i in 0..2 {
        let end_auction_msg = ExecuteMsg::EndAuction {
            stock_id: stock_ids[i],
        };
        app.execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &end_auction_msg,
            &[],
        )
        .unwrap();
    }

    // Query expired active auctions again
    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify now only 3 expired auctions are still active
    assert_eq!(response.stocks.len(), 3);

    // Verify the manually ended auctions are not in the results
    for stock in &response.stocks {
        assert!(stock.id != stock_ids[0] && stock.id != stock_ids[1]);
    }
}

#[test]
fn test_get_expired_active_auctions_empty() {
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

    // Query expired active auctions when none exist
    let query_msg = QueryMsg::GetExpiredActiveAuctions {
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
fn test_get_expired_active_auctions_partially_expired() {
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

    // Create 3 first stocks
    let influencer = Addr::unchecked("influencer1");
    let mut stock_ids = Vec::new();

    for i in 1..=3 {
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

    // Start auctions for all 3 stocks
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

    // Verify no expired auctions initially
    let query_msg = QueryMsg::GetExpiredActiveAuctions {
        limit: None,
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    assert_eq!(response.stocks.len(), 0);

    // Advance block time to artifically advance time for the next 3 auctions
    // (by half the expiry period of the first auctions)
    app.update_block(|block| {
        block.time = block.time.plus_seconds(12 * 60 * 60); // 12 hours
    });

    // Create 3 next stocks
    let stock_id_len = stock_ids.len();

    for i in 1..=3 {
        let ticker = format!("INF1_{}", stock_id_len + i);
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

    // Start auctions for next 3 stocks
    for &stock_id in &stock_ids[3..] {
        let start_auction_msg = ExecuteMsg::StartAuction { stock_id };
        app.execute_contract(
            influencer.clone(),
            contract_addr.clone(),
            &start_auction_msg,
            &[],
        )
        .unwrap();
    }

    // Advance time further to expire only the first 3 auctions
    app.update_block(|block| {
        block.time = block.time.plus_seconds(12 * 60 * 60 + 1); // 12 hours + 1 second
    });

    // Query expired active auctions
    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify only the first 3 auctions are expired
    assert_eq!(response.stocks.len(), 3);

    // Verify the correct stocks are expired
    for stock in &response.stocks {
        assert!(stock_ids[0..3].contains(&stock.id));
        assert_eq!(stock.auction_active, 1);
        assert!(stock.auction_end.unwrap() <= app.block_info().time.nanos() / 1_000_000);
    }

    // Verify the other 3 auctions are still active but not expired
    for i in 3..6 {
        let query_stock_msg = QueryMsg::GetStockById {
            stock_id: stock_ids[i],
        };
        let stock_response: GetStockByIdResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &query_stock_msg)
            .unwrap();

        assert_eq!(stock_response.stock.auction_active, 1);
        assert!(
            stock_response.stock.auction_end.unwrap() > app.block_info().time.nanos() / 1_000_000
        );
    }

    // Advance time to expire all remaining auctions
    app.update_block(|block| {
        block.time = block.time.plus_seconds(12 * 60 * 60 + 1); // another 12 hours + 1 second
    });

    // Query expired active auctions again
    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify all 6 auctions are now expired
    assert_eq!(response.stocks.len(), 6);
}

#[test]
fn test_get_expired_active_auctions_with_limit() {
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

    // Advance block time to expire all 5 auctions
    app.update_block(|block| {
        block.time = block.time.plus_seconds(24 * 60 * 60 + 1); // 24 hours + 1 second
    });

    // Query with limit
    let limit = 3;
    let query_msg = QueryMsg::GetExpiredActiveAuctions {
        limit: Some(limit),
        start_after: None,
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify number of stocks respects limit
    assert_eq!(response.stocks.len(), limit);

    // Verify all returned stocks have active but expired auctions
    for stock in &response.stocks {
        assert_eq!(stock.auction_active, 1);
        assert!(stock.auction_end.unwrap() <= app.block_info().time.nanos() / 1_000_000);
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }
}

#[test]
fn test_get_expired_active_auctions_with_start_after() {
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

    // Advance block time to expire all 5 auctions
    app.update_block(|block| {
        block.time = block.time.plus_seconds(24 * 60 * 60 + 1); // 24 hours + 1 second
    });

    // Get ID of the 3rd stock for start_after parameter
    let start_after = stock_ids[2];

    let query_msg = QueryMsg::GetExpiredActiveAuctions {
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
    // and have active but expired auctions
    for stock in &response.stocks {
        assert!(stock.id < start_after);
        assert_eq!(stock.auction_active, 1);
        assert!(stock.auction_end.unwrap() <= app.block_info().time.nanos() / 1_000_000);
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }
}

#[test]
fn test_get_expired_active_auctions_with_limit_and_start_after() {
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

    // Create 10 stocks with the same influencer
    let influencer = Addr::unchecked("influencer1");
    let mut stock_ids = Vec::new();

    for i in 1..=10 {
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

    // Start auctions for all 10 stocks
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

    // Advance block time to expire all 10 auctions
    app.update_block(|block| {
        block.time = block.time.plus_seconds(24 * 60 * 60 + 1); // 24 hours + 1 second
    });

    // Get ID of the 5th stock for start_after parameter
    let start_after = stock_ids[4];
    let limit = 3;

    let query_msg = QueryMsg::GetExpiredActiveAuctions {
        limit: Some(limit),
        start_after: Some(start_after),
    };

    let response: GetStocksResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    // Verify we get only the number of stocks specified by limit
    assert_eq!(response.stocks.len(), limit);

    // Verify all returned stocks have ID less than start_after
    // and have active but expired auctions
    for stock in &response.stocks {
        assert!(stock.id < start_after);
        assert_eq!(stock.auction_active, 1);
        assert!(stock.auction_end.unwrap() <= app.block_info().time.nanos() / 1_000_000);
    }

    // Verify stocks are in descending order by ID
    for i in 0..response.stocks.len() - 1 {
        assert!(response.stocks[i].id > response.stocks[i + 1].id);
    }
}
