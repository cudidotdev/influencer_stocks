use common::{contract_code, setup_app};
use cosmwasm_std::coins;
use cw_multi_test::Executor;
use influencer_stocks::{
    contract::DENOM,
    msg::{
        ExecuteMsg, GetBuyOrdersResponse, GetSalesResponse, GetSellOrdersResponse,
        GetSharesResponse, InstantiateMsg, OrderSort, QueryMsg,
    },
};

mod common;

#[test]
fn test_create_multiple_sell_multiple_buy_orders() {
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
    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &end_auction_msg,
        &[],
    )
    .unwrap();

    // bidder2 creates mulitple sell orders at varing price

    // 1000 shares @ 20 per share
    let create_sell_order_msg = ExecuteMsg::CreateSellOrder {
        stock_id,
        price_per_share: 20,
        shares: 1000,
    };

    app.execute_contract(
        bidder2.clone(),
        contract_addr.clone(),
        &create_sell_order_msg,
        &[],
    )
    .unwrap();

    // 5_000 shares @ 30 per share
    let create_sell_order_msg = ExecuteMsg::CreateSellOrder {
        stock_id,
        price_per_share: 30,
        shares: 5000,
    };

    app.execute_contract(
        bidder2.clone(),
        contract_addr.clone(),
        &create_sell_order_msg,
        &[],
    )
    .unwrap();

    // 5_000 shares @ 25 per share
    let create_sell_order_msg = ExecuteMsg::CreateSellOrder {
        stock_id,
        price_per_share: 25,
        shares: 5000,
    };

    app.execute_contract(
        bidder2.clone(),
        contract_addr.clone(),
        &create_sell_order_msg,
        &[],
    )
    .unwrap();

    //verify that sell order recorded
    let get_sell_order_msg = QueryMsg::GetOpenSellOrdersByStock {
        stock_id,
        sort_by: OrderSort::PriceAsc,
    };

    let res: GetSellOrdersResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_sell_order_msg)
        .unwrap();

    let orders = res.orders;

    assert_eq!(orders.len(), 3);

    assert_eq!(orders[0].id, 1);
    assert_eq!(orders[1].id, 3);
    assert_eq!(orders[2].id, 2);

    let user = app.api().addr_make("user");

    app.send_tokens(vault.clone(), user.clone(), &coins(10_000_000, DENOM))
        .unwrap();

    // bidder2 balance before buy order
    let bidder2_balance_pre = app
        .wrap()
        .query_balance(bidder2.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    // user balance before buy order
    let user_balance_pre = app
        .wrap()
        .query_balance(user.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    let create_buy_order_msg = ExecuteMsg::CreateBuyOrder {
        stock_id,
        price_per_share: 30,
        shares: 5000,
    };

    app.execute_contract(
        user.clone(),
        contract_addr.clone(),
        &create_buy_order_msg,
        &coins(150_000, DENOM),
    )
    .unwrap();

    // verify bidder2 balance increased by (20*1000 + 25*4000)
    let bidder2_balance = app
        .wrap()
        .query_balance(bidder2.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(bidder2_balance, bidder2_balance_pre + 20 * 1000 + 25 * 4000);

    // verify that user was refunded
    let user_balance = app
        .wrap()
        .query_balance(user.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(user_balance_pre - user_balance, 20 * 1000 + 25 * 4000);

    //verify there are no open buy order
    let get_buy_orders_msg = QueryMsg::GetOpenBuyOrdersByStock {
        stock_id,
        sort_by: OrderSort::PriceAsc,
    };

    let res: GetSellOrdersResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_buy_orders_msg)
        .unwrap();

    assert_eq!(res.orders.len(), 0);

    //verify sell orders
    let get_sell_orders_msg = QueryMsg::GetOpenSellOrdersByStock {
        stock_id,
        sort_by: OrderSort::PriceAsc,
    };

    let res: GetSellOrdersResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_sell_orders_msg)
        .unwrap();

    assert_eq!(res.orders.len(), 2);

    let orders = res.orders;

    assert_eq!(orders[0].price_per_share, 25);
    assert_eq!(orders[0].sold_shares, 4000);

    assert_eq!(orders[1].price_per_share, 30);
    assert_eq!(orders[1].sold_shares, 0);

    //verify that sales where recorded
    let get_sales_msg = QueryMsg::GetSalesByStock { stock_id };

    let res: GetSalesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_sales_msg)
        .unwrap();

    let sales = res.sales;

    // 2 after bidding and 2 in the order
    assert_eq!(sales.len(), 4);

    assert_eq!(sales[0].from, bidder2.clone());
    assert_eq!(sales[0].to, user.clone());
    assert_eq!(sales[0].no_of_shares, 4000);
    assert_eq!(sales[0].price_per_share, 25);

    assert_eq!(sales[1].from, bidder2.clone());
    assert_eq!(sales[1].to, user.clone());
    assert_eq!(sales[1].no_of_shares, 1000);
    assert_eq!(sales[1].price_per_share, 20);

    assert_eq!(sales[2].from, influencer.clone());
    assert_eq!(sales[2].to, bidder2.clone());
    assert_eq!(sales[2].no_of_shares, 950_000);
    assert_eq!(sales[2].price_per_share, 11);

    assert_eq!(sales[3].from, influencer.clone());
    assert_eq!(sales[3].to, bidder1.clone());
    assert_eq!(sales[3].no_of_shares, 50_000);
    assert_eq!(sales[3].price_per_share, 10);

    // confirm the users shares increased and bidder2 shares decreased

    let get_shares_msg = QueryMsg::GetSharesByOwner {
        owner: user.clone(),
    };

    let res: GetSharesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_shares_msg)
        .unwrap();

    let shares = res.shares;

    assert_eq!(shares[0].stock_id, stock_id);
    assert_eq!(shares[0].no_of_shares, 5000);

    let get_shares_msg = QueryMsg::GetSharesByOwner {
        owner: bidder2.clone(),
    };

    let res: GetSharesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_shares_msg)
        .unwrap();

    let shares = res.shares;

    assert_eq!(shares[0].stock_id, stock_id);
    assert_eq!(shares[0].no_of_shares, 950_000 - 5000);

    // create buy orders by user

    let create_buy_order_msg = ExecuteMsg::CreateBuyOrder {
        stock_id,
        price_per_share: 10,
        shares: 5000,
    };

    app.execute_contract(
        user.clone(),
        contract_addr.clone(),
        &create_buy_order_msg,
        &coins(50_000, DENOM),
    )
    .unwrap();

    let create_buy_order_msg = ExecuteMsg::CreateBuyOrder {
        stock_id,
        price_per_share: 5,
        shares: 5000,
    };

    app.execute_contract(
        user.clone(),
        contract_addr.clone(),
        &create_buy_order_msg,
        &coins(25_000, DENOM),
    )
    .unwrap();

    let create_buy_order_msg = ExecuteMsg::CreateBuyOrder {
        stock_id,
        price_per_share: 20,
        shares: 1000,
    };

    app.execute_contract(
        user.clone(),
        contract_addr.clone(),
        &create_buy_order_msg,
        &coins(20_000, DENOM),
    )
    .unwrap();

    // verify contract balance before sell order
    let contract_balance_pre = app
        .wrap()
        .query_balance(contract_addr.clone(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(contract_balance_pre, 95_000);

    // Bidder balance before sell
    let bidder_balance_pre = app
        .wrap()
        .query_balance(bidder1.clone(), DENOM)
        .unwrap()
        .amount
        .u128();

    // create sell orders from bidder

    // 5_000 shares @ 20 per share
    let create_sell_order_msg = ExecuteMsg::CreateSellOrder {
        stock_id,
        price_per_share: 2,
        shares: 5000,
    };

    app.execute_contract(
        bidder1.clone(),
        contract_addr.clone(),
        &create_sell_order_msg,
        &[],
    )
    .unwrap();

    // verify contract balance
    let contract_balance = app
        .wrap()
        .query_balance(contract_addr.clone(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(
        contract_balance_pre - contract_balance,
        20 * 1000 + 10 * 4000
    );

    // verify bidder balance
    let bidder_balance = app
        .wrap()
        .query_balance(bidder1.clone(), DENOM)
        .unwrap()
        .amount
        .u128();

    assert_eq!(bidder_balance, bidder_balance_pre + (20 * 1000 + 10 * 4000));

    // verify buy orders
    let get_buy_orders_msg = QueryMsg::GetOpenBuyOrdersByStock {
        stock_id,
        sort_by: OrderSort::PriceDesc,
    };

    let res: GetBuyOrdersResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_buy_orders_msg)
        .unwrap();

    assert_eq!(res.orders.len(), 2);

    let orders = res.orders;

    assert_eq!(orders[0].price_per_share, 10);
    assert_eq!(orders[0].bought_shares, 4000);

    assert_eq!(orders[1].price_per_share, 5);
    assert_eq!(orders[1].bought_shares, 0);
}
