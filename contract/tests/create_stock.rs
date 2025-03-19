use common::{contract_code, setup_app};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use influencer_stocks::{
    contract::execute::stocks::TOTAL_SHARES,
    msg::{ExecuteMsg, GetStockByIdResponse, InstantiateMsg, QueryMsg},
    state::Stock,
};

mod common;

#[test]
fn test_create_and_query_stock_by_id() {
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

    let influencer = Addr::unchecked("influencer");

    // Create a stock
    let ticker = "INFL1".to_string();
    let create_msg = ExecuteMsg::CreateStock {
        ticker: ticker.clone(),
    };

    let res = app
        .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
        .unwrap();

    let attr = res.custom_attrs(1);

    let stock_id: u64 = attr
        .iter()
        .find(|attr| attr.key == "stock_id".to_owned())
        .unwrap()
        .value
        //convert from string to u64
        .parse()
        .unwrap();

    // Verify stock id
    assert_eq!(stock_id, 1);

    let query_msg = QueryMsg::GetStockById { stock_id };

    let response: GetStockByIdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &query_msg)
        .unwrap();

    assert_eq!(
        GetStockByIdResponse {
            stock: Stock {
                id: stock_id,
                ticker,
                influencer,
                total_shares: TOTAL_SHARES,
                auction_start: None,
                auction_end: None,
                marked_as_active_auction: false,
                created_at: response.clone().stock.created_at
            }
        },
        response
    );
}

#[test]
fn test_create_multiple_stocks() {
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

    // Create 3 stocks with different influencers
    let influencers = vec![
        Addr::unchecked("influencer1"),
        Addr::unchecked("influencer2"),
        Addr::unchecked("influencer3"),
    ];

    let tickers = vec!["INFL1", "INFL2", "INFL3"];

    // Create stocks and verify IDs are sequential
    for (i, (influencer, ticker)) in influencers.iter().zip(tickers.iter()).enumerate() {
        let create_msg = ExecuteMsg::CreateStock {
            ticker: ticker.to_string(),
        };

        let res = app
            .execute_contract(influencer.clone(), contract_addr.clone(), &create_msg, &[])
            .unwrap();

        // Check stock_id
        let attrs = res.custom_attrs(1);
        let stock_id_attr = attrs.iter().find(|attr| attr.key == "stock_id").unwrap();
        assert_eq!(stock_id_attr.value, (i + 1).to_string());

        // Query to verify stock was properly created
        let query_msg = QueryMsg::GetStockById {
            stock_id: (i + 1) as u64,
        };
        let response: GetStockByIdResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &query_msg)
            .unwrap();

        assert_eq!(response.stock.id, (i + 1) as u64);
        assert_eq!(response.stock.ticker, ticker.to_string());
        assert_eq!(response.stock.influencer, influencer.clone());
    }
}
