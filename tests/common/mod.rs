use cosmwasm_std::{coins, Addr, Attribute, Empty};
use cw_multi_test::{App, Contract, ContractWrapper};
use influencer_stocks::contract;

pub const DENOM: &str = "uhuahua";

// Create test environment with initial balances
pub fn setup_app() -> (App, Addr) {
    let mut app = App::default();

    // Create vault for tokens
    let vault = Addr::unchecked("vault");

    // Add funds to Vault
    app.init_modules(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &vault, coins(1_000_000, DENOM))
            .unwrap();
    });

    (app, vault)
}

// Helper function to get contract code
pub fn contract_code() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(contract::execute, contract::instantiate, contract::query);
    Box::new(contract)
}
