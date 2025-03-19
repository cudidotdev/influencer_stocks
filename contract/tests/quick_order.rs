use common::{contract_code, setup_app};
use cosmwasm_std::coins;
use cw_multi_test::Executor;
use influencer_stocks::{
    contract::DENOM,
    msg::{
        ExecuteMsg, GetBuyOrdersResponse, GetBuyPriceResponse, GetSalesResponse,
        GetSellOrdersResponse, GetSharesResponse, InstantiateMsg, OrderSort, QueryMsg,
    },
};

mod common;

#[test]
fn test_quick_buy() {
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

    // Create multiple sell orders
    app.execute_contract(
        bidder1.clone(),
        contract_addr.clone(),
        &ExecuteMsg::CreateSellOrder {
            stock_id,
            price_per_share: 12,
            shares: 20_000,
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        bidder2.clone(),
        contract_addr.clone(),
        &ExecuteMsg::CreateSellOrder {
            stock_id,
            price_per_share: 15,
            shares: 30_000,
        },
        &[],
    )
    .unwrap();

    // Create a buyer
    let buyer = app.api().addr_make("buyer");
    app.send_tokens(vault.clone(), buyer.clone(), &coins(1_000_000, DENOM))
        .unwrap();

    // Get buyer's initial balance
    let buyer_balance_pre = app
        .wrap()
        .query_balance(buyer.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    // Get sellers' initial balances
    let bidder1_balance_pre = app
        .wrap()
        .query_balance(bidder1.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    let bidder2_balance_pre = app
        .wrap()
        .query_balance(bidder2.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    // Verify Buy price
    let get_buy_price_msg = QueryMsg::GetBuyPrice {
        stock_id,
        requested_shares: 25_000,
    };

    let res: GetBuyPriceResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_buy_price_msg)
        .unwrap();

    assert_eq!(
        res.total_price.parse::<u128>().unwrap(),
        12 * 20_000 + 15 * 5000
    );

    assert_eq!(
        res.price_per_share.parse::<u128>().unwrap(),
        (12 * 20_000 + 15 * 5000) / 25_000
    );

    // Verify that we can't buy more than available
    let get_buy_price_msg = QueryMsg::GetBuyPrice {
        stock_id,
        requested_shares: 50_000 + 1,
    };

    let res = app
        .wrap()
        .query_wasm_smart::<GetBuyPriceResponse>(contract_addr.clone(), &get_buy_price_msg);

    assert!(res.is_err());

    // Quick buy shares with 5% slippage
    let quick_buy_msg = ExecuteMsg::QuickBuy {
        stock_id,
        shares: 25_000,
        slippage: 5,
    };

    app.execute_contract(
        buyer.clone(),
        contract_addr.clone(),
        &quick_buy_msg,
        &coins(400_000, DENOM), // More than enough to cover
    )
    .unwrap();

    // Check balances after quick buy
    let buyer_balance_post = app
        .wrap()
        .query_balance(buyer.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    let bidder1_balance_post = app
        .wrap()
        .query_balance(bidder1.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    let bidder2_balance_post = app
        .wrap()
        .query_balance(bidder2.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    // Verify buyer paid correct amount
    let buyer_paid = buyer_balance_pre - buyer_balance_post;
    let expected_cost = 20_000 * 12 + 5_000 * 15;
    assert_eq!(buyer_paid, expected_cost);

    // Verify sellers received correct amounts
    assert_eq!(bidder1_balance_post - bidder1_balance_pre, 20_000 * 12);
    assert_eq!(bidder2_balance_post - bidder2_balance_pre, 5_000 * 15);

    // Verify buyer received shares
    let get_shares_msg = QueryMsg::GetSharesByOwner {
        owner: buyer.clone(),
    };

    let res: GetSharesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_shares_msg)
        .unwrap();

    let shares = res.shares;
    assert_eq!(shares[0].stock_id, stock_id);
    assert_eq!(shares[0].no_of_shares, 25_000);

    // Verify that sell orders were updated
    let get_sell_orders_msg = QueryMsg::GetOpenSellOrdersByStock {
        stock_id,
        sort_by: OrderSort::PriceAsc,
    };

    let res: GetSellOrdersResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_sell_orders_msg)
        .unwrap();

    let orders = res.orders;
    assert_eq!(orders.len(), 1); // Only one order should remain
    assert_eq!(orders[0].price_per_share, 15);
    assert_eq!(orders[0].sold_shares, 5_000);
    assert_eq!(orders[0].available_shares, 30_000);

    // Verify sales were recorded
    let get_sales_msg = QueryMsg::GetSalesByStock { stock_id };
    let res: GetSalesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_sales_msg)
        .unwrap();

    let sales = res.sales;
    assert_eq!(sales.len(), 4); // 2 from auction + 2 from quick buy

    // Verify the latest sales
    assert_eq!(sales[0].from, bidder2.clone());
    assert_eq!(sales[0].to, buyer.clone());
    assert_eq!(sales[0].no_of_shares, 5_000);
    assert_eq!(sales[0].price_per_share, 15);

    assert_eq!(sales[1].from, bidder1.clone());
    assert_eq!(sales[1].to, buyer.clone());
    assert_eq!(sales[1].no_of_shares, 20_000);
    assert_eq!(sales[1].price_per_share, 12);
}

#[test]
fn test_quick_sell() {
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

    // Create seller and buyer
    let seller = app.api().addr_make("seller");
    app.send_tokens(vault.clone(), seller.clone(), &coins(1_000_000, DENOM))
        .unwrap();

    // Place Bid by seller
    app.execute_contract(
        seller.clone(),
        contract_addr.clone(),
        &ExecuteMsg::PlaceBid {
            stock_id,
            price_per_share: 10,
            shares: 100_000,
        },
        &coins(1_000_000, DENOM),
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

    // Create buyers and fund them
    let buyer1 = app.api().addr_make("buyer1");
    let buyer2 = app.api().addr_make("buyer2");

    app.send_tokens(vault.clone(), buyer1.clone(), &coins(300_000, DENOM))
        .unwrap();

    app.send_tokens(vault.clone(), buyer2.clone(), &coins(500_000, DENOM))
        .unwrap();

    // Create buy orders
    app.execute_contract(
        buyer1.clone(),
        contract_addr.clone(),
        &ExecuteMsg::CreateBuyOrder {
            stock_id,
            price_per_share: 12,
            shares: 10_000,
        },
        &coins(120_000, DENOM),
    )
    .unwrap();

    app.execute_contract(
        buyer2.clone(),
        contract_addr.clone(),
        &ExecuteMsg::CreateBuyOrder {
            stock_id,
            price_per_share: 13,
            shares: 20_000,
        },
        &coins(260_000, DENOM),
    )
    .unwrap();

    // Get seller's initial balance
    let seller_balance_pre = app
        .wrap()
        .query_balance(seller.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    // Quick sell shares with 5% slippage
    let quick_sell_msg = ExecuteMsg::QuickSell {
        stock_id,
        shares: 25_000,
        price_per_share: 12, // Minimum price
        slippage: 5,
    };

    app.execute_contract(seller.clone(), contract_addr.clone(), &quick_sell_msg, &[])
        .unwrap();

    // Check seller's balance after quick sell
    let seller_balance_post = app
        .wrap()
        .query_balance(seller.clone().to_string(), DENOM)
        .unwrap()
        .amount
        .u128();

    // Verify seller received correct amount
    let seller_received = seller_balance_post - seller_balance_pre;
    let expected_revenue = 20_000 * 13 + 5_000 * 12;
    assert_eq!(seller_received, expected_revenue);

    // Verify seller's shares were reduced
    let get_shares_msg = QueryMsg::GetSharesByOwner {
        owner: seller.clone(),
    };

    let res: GetSharesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_shares_msg)
        .unwrap();

    let shares = res.shares;
    assert_eq!(shares[0].stock_id, stock_id);
    assert_eq!(shares[0].no_of_shares, 100_000 - 25_000);

    // Verify that buy orders were updated
    let get_buy_orders_msg = QueryMsg::GetOpenBuyOrdersByStock {
        stock_id,
        sort_by: OrderSort::PriceDesc,
    };

    let res: GetBuyOrdersResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_buy_orders_msg)
        .unwrap();

    let orders = res.orders;
    assert_eq!(orders.len(), 1); // Only one order should remain
    assert_eq!(orders[0].price_per_share, 12);
    assert_eq!(orders[0].bought_shares, 5_000);
    assert_eq!(orders[0].requested_shares, 10_000);

    // Verify buyers received shares
    let get_shares_msg = QueryMsg::GetSharesByOwner {
        owner: buyer1.clone(),
    };

    let res: GetSharesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_shares_msg)
        .unwrap();

    let shares = res.shares;
    assert_eq!(shares[0].stock_id, stock_id);
    assert_eq!(shares[0].no_of_shares, 5_000);

    let get_shares_msg = QueryMsg::GetSharesByOwner {
        owner: buyer2.clone(),
    };

    let res: GetSharesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_shares_msg)
        .unwrap();

    let shares = res.shares;
    assert_eq!(shares[0].stock_id, stock_id);
    assert_eq!(shares[0].no_of_shares, 20_000);

    // Verify sales were recorded
    let get_sales_msg = QueryMsg::GetSalesByStock { stock_id };
    let res: GetSalesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &get_sales_msg)
        .unwrap();

    let sales = res.sales;

    // 1 from auction + 2 from quick sell 1 from balance during auction (influencer)
    assert_eq!(sales.len(), 4);

    // Verify the latest sales
    assert_eq!(sales[0].from, seller.clone());
    assert_eq!(sales[0].to, buyer1.clone());
    assert_eq!(sales[0].no_of_shares, 5_000);
    assert_eq!(sales[0].price_per_share, 12);

    assert_eq!(sales[1].from, seller.clone());
    assert_eq!(sales[1].to, buyer2.clone());
    assert_eq!(sales[1].no_of_shares, 20_000);
    assert_eq!(sales[1].price_per_share, 13);
}

#[test]
fn test_quick_buy_insufficient_funds() {
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
    let stock_id: u64 = res
        .events
        .iter()
        .flat_map(|event| &event.attributes)
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    // Start and end auction
    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::StartAuction { stock_id },
        &[],
    )
    .unwrap();

    let seller = app.api().addr_make("seller");
    app.send_tokens(vault.clone(), seller.clone(), &coins(1_000_000, DENOM))
        .unwrap();

    app.execute_contract(
        seller.clone(),
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
        influencer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::EndAuction { stock_id },
        &[],
    )
    .unwrap();

    // Create a sell order
    app.execute_contract(
        seller.clone(),
        contract_addr.clone(),
        &ExecuteMsg::CreateSellOrder {
            stock_id,
            price_per_share: 15,
            shares: 50_000,
        },
        &[],
    )
    .unwrap();

    // Create a buyer with insufficient funds
    let buyer = app.api().addr_make("buyer");
    app.send_tokens(vault.clone(), buyer.clone(), &coins(100_000, DENOM))
        .unwrap();

    // Attempt to quick buy with insufficient funds
    let quick_buy_msg = ExecuteMsg::QuickBuy {
        stock_id,
        shares: 10_000,
        slippage: 5,
    };

    let err = app
        .execute_contract(
            buyer.clone(),
            contract_addr.clone(),
            &quick_buy_msg,
            &coins(100_000, DENOM), // Not enough for 10,000 shares at 15 per share
        )
        .unwrap_err();

    // Verify the error
    assert!(err.root_cause().to_string().contains("Insufficient funds"));
}

#[test]
fn test_quick_sell_slippage_exceeded() {
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
    let stock_id: u64 = res
        .events
        .iter()
        .flat_map(|event| &event.attributes)
        .find(|attr| attr.key == "stock_id")
        .unwrap()
        .value
        .parse()
        .unwrap();

    // Start and end auction
    app.execute_contract(
        influencer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::StartAuction { stock_id },
        &[],
    )
    .unwrap();

    let seller = app.api().addr_make("seller");
    app.send_tokens(vault.clone(), seller.clone(), &coins(1_000_000, DENOM))
        .unwrap();

    app.execute_contract(
        seller.clone(),
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
        influencer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::EndAuction { stock_id },
        &[],
    )
    .unwrap();

    // Create a buyer with funds
    let buyer = app.api().addr_make("buyer");
    app.send_tokens(vault.clone(), buyer.clone(), &coins(100_000, DENOM))
        .unwrap();

    // Create a buy order at a low price
    app.execute_contract(
        buyer.clone(),
        contract_addr.clone(),
        &ExecuteMsg::CreateBuyOrder {
            stock_id,
            price_per_share: 8,
            shares: 10_000,
        },
        &coins(80_000, DENOM),
    )
    .unwrap();

    // Try to quick sell at a high price with low slippage
    let quick_sell_msg = ExecuteMsg::QuickSell {
        stock_id,
        shares: 10_000,
        price_per_share: 15, // Much higher than the available buy order
        slippage: 5,         // Only 5% slippage
    };

    let err = app
        .execute_contract(seller.clone(), contract_addr.clone(), &quick_sell_msg, &[])
        .unwrap_err();

    // Verify the error
    assert!(err
        .root_cause()
        .to_string()
        .contains("Actual price fell below slippage tolerance"));
}
