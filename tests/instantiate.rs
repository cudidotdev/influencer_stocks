use common::{contract_code, setup_app};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use influencer_stocks::msg::InstantiateMsg;

mod common;

#[test]
fn test_instantiate() {
    let (mut app, owner) = setup_app();

    let code_id = app.store_code(contract_code());

    // Instantiate the contract
    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner,
            &InstantiateMsg {},
            &[],
            "Influencer Stocks",
            None,
        )
        .unwrap();

    // Verify contract exists
    assert_ne!(contract_addr, Addr::unchecked(""));
}
