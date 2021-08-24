#![cfg(test)]
use crate::msg::{Cw20HookMsg, ExecuteMsg, QueryMsg};
use crate::state::{Staker, State};
use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{to_binary, Addr, Empty, Uint128};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg};
use cw_multi_test::{App, Contract, ContractWrapper, SimpleBank};
use std::ops::Add;

fn mock_app() -> App {
    let env = mock_env();
    let api = Box::new(MockApi::default());
    let bank = SimpleBank {};

    App::new(api, env.block, bank, || Box::new(MockStorage::new()))
}

pub fn gov_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn cw20_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

struct Vars {
    router: App,
    token_addr: Addr,
    gov_addr: Addr,
}

fn init(stakers_balances: &Vec<(Addr, Uint128)>) -> Vars {
    let mut router = mock_app();
    let gov_owner_addr = Addr::unchecked("gov_owner");

    //Instantiate Contracts Code
    let cw20_code_id = router.store_code(cw20_contract());
    let gov_contract_id = router.store_code(gov_contract());

    let initial_balances: Vec<Cw20Coin> = stakers_balances
        .iter()
        .map(|s| Cw20Coin {
            address: s.0.to_string(),
            amount: s.1,
        })
        .collect();

    //Instantiate Token
    let cw20_init_msg = cw20_base::msg::InstantiateMsg {
        name: "LocalTerra's Token".to_string(),
        symbol: "LOCAL".to_string(),
        decimals: 6,
        initial_balances,
        mint: None,
    };
    let token_addr = router
        .instantiate_contract(
            cw20_code_id,
            gov_owner_addr.clone(),
            &cw20_init_msg,
            &[],
            "LOCAL",
        )
        .unwrap();

    //Instantiate Gov Contract
    let gov_init_msg = crate::msg::InstantiateMsg {
        gov_token_addr: token_addr.clone(),
    };
    let gov_contract_res =
        router.instantiate_contract(gov_contract_id, gov_owner_addr, &gov_init_msg, &[], "GOV");
    assert!(gov_contract_res.is_ok());

    let gov_addr = gov_contract_res.unwrap();

    Vars {
        router,
        token_addr,
        gov_addr,
    }
}

impl Vars {
    pub fn load_gov_state(&self) -> State {
        self.router
            .wrap()
            .query_wasm_smart(&self.gov_addr, &QueryMsg::State {})
            .unwrap()
    }

    pub fn load_staker(&self, address: String) -> Staker {
        self.router
            .wrap()
            .query_wasm_smart(&self.gov_addr, &QueryMsg::Staker { address })
            .unwrap()
    }
}

#[test]
pub fn proper_init() {
    let staker_a_addr = Addr::unchecked("staker_a");
    let staker_a_balance = Uint128::new(1000u128);
    let stakers = vec![(staker_a_addr.clone(), staker_a_balance.clone())];
    let vars = &mut init(&stakers);

    assert!(!vars.token_addr.as_str().is_empty());
    let token = Cw20Contract(vars.token_addr.clone());

    //Check staker balance
    let staker_balance = token.balance(&vars.router, staker_a_addr.clone()).unwrap();
    assert_eq!(staker_balance, Uint128::from(1000u128));

    //Send 1000 tokens to the Staking contract
    let send_msg = Cw20ExecuteMsg::Send {
        contract: vars.gov_addr.to_string(),
        amount: staker_balance,
        msg: Some(to_binary(&Cw20HookMsg::StakeTokens {}).unwrap()),
    };
    let res = vars.router.execute_contract(
        staker_a_addr.clone(),
        vars.token_addr.clone(),
        &send_msg,
        &[],
    );
    assert!(res.is_ok());

    //Check Gov Contract and Staker balances.
    let gov_contract_balance = token.balance(&vars.router, vars.gov_addr.clone()).unwrap();
    assert_eq!(gov_contract_balance, Uint128::from(1000u128));

    //Verify Gov Contract State
    let state = vars.load_gov_state();
    assert_eq!(state.total_shares, staker_balance);
}

#[test]
pub fn increase_stake_test() {
    let factory_addr = Addr::unchecked("factory");
    let factory_balance = Uint128::new(1000 * 1000u128);
    let staker_a_addr = Addr::unchecked("staker_a");
    let staker_a_balance = Uint128::new(1000u128);
    let staker_b_addr = Addr::unchecked("staker_b");
    let staker_b_balance = Uint128::new(1250u128);
    let balances = vec![
        (factory_addr.clone(), factory_balance.clone()),
        (staker_a_addr.clone(), staker_a_balance.clone()),
        (staker_b_addr.clone(), staker_b_balance.clone()),
    ];
    let vars = &mut init(&balances);

    //Send tokens from Staker A to the Staking contract
    let send_msg = Cw20ExecuteMsg::Send {
        contract: vars.gov_addr.to_string(),
        amount: staker_a_balance.clone(),
        msg: Some(to_binary(&Cw20HookMsg::StakeTokens {}).unwrap()),
    };
    let res = vars.router.execute_contract(
        staker_a_addr.clone(),
        vars.token_addr.clone(),
        &send_msg,
        &[],
    );
    assert!(res.is_ok());

    //Check Gov Contract and Staker balances.
    let token = Cw20Contract(vars.token_addr.clone());
    let gov_contract_balance = token.balance(&vars.router, vars.gov_addr.clone()).unwrap();
    assert_eq!(gov_contract_balance, Uint128::from(1000u128));

    //Verify Gov Contract State
    let state = vars.load_gov_state();
    assert_eq!(state.total_shares, staker_a_balance.clone());

    //Send from factory to Staking contract to simulate Rewards Deposit
    let send_msg = Cw20ExecuteMsg::Send {
        contract: vars.gov_addr.to_string(),
        amount: Uint128::new(100u128),
        msg: Some(to_binary(&Cw20HookMsg::DepositRewards {}).unwrap()),
    };
    let res = vars.router.execute_contract(
        factory_addr.clone(),
        vars.token_addr.clone(),
        &send_msg,
        &[],
    );
    assert!(res.is_ok());

    //Send tokens from Staker B to the Staking contract
    let send_msg = Cw20ExecuteMsg::Send {
        contract: vars.gov_addr.to_string(),
        amount: staker_b_balance.clone(),
        msg: Some(to_binary(&Cw20HookMsg::StakeTokens {}).unwrap()),
    };
    let res = vars.router.execute_contract(
        staker_b_addr.clone(),
        vars.token_addr.clone(),
        &send_msg,
        &[],
    );
    assert!(res.is_ok());

    let staker_a = vars.load_staker(staker_a_addr.to_string());
    let staker_b = vars.load_staker(staker_b_addr.to_string());

    //Verify Gov Contract State
    let state = vars.load_gov_state();
    assert_eq!(state.total_shares, staker_a.shares.add(staker_b.shares));
}

#[test]
fn withdraw_test() {
    let factory_addr = Addr::unchecked("factory");
    let factory_balance = Uint128::new(1000 * 1000u128);
    let staker_a_addr = Addr::unchecked("staker_a");
    let staker_a_balance = Uint128::new(1000u128);

    let balances = vec![
        (factory_addr.clone(), factory_balance.clone()),
        (staker_a_addr.clone(), staker_a_balance.clone()),
    ];
    let vars = &mut init(&balances);
    let token = Cw20Contract(vars.token_addr.clone());

    //Send tokens from Staker A to the Staking contract
    let send_msg = Cw20ExecuteMsg::Send {
        contract: vars.gov_addr.to_string(),
        amount: staker_a_balance.clone(),
        msg: Some(to_binary(&Cw20HookMsg::StakeTokens {}).unwrap()),
    };
    let res = vars.router.execute_contract(
        staker_a_addr.clone(),
        vars.token_addr.clone(),
        &send_msg,
        &[],
    );
    assert!(res.is_ok());
    let staker_balance = token
        .balance(&vars.router, staker_a_addr.to_string())
        .unwrap();
    assert_eq!(staker_balance, Uint128::zero());
    let state = vars.load_gov_state();
    let staker_a = vars.load_staker(staker_a_addr.to_string());
    assert_ne!(staker_a.shares, Uint128::zero());
    assert_eq!(state.total_shares, staker_a.shares);

    //Withdraw tokens
    let withdraw_msg = ExecuteMsg::Withdraw {
        shares: staker_a.shares,
    };
    let res = vars.router.execute_contract(
        staker_a_addr.clone(),
        vars.gov_addr.clone(),
        &withdraw_msg,
        &[],
    );
    assert!(res.is_ok());
    let state = vars.load_gov_state();
    let staker_a = vars.load_staker(staker_a_addr.to_string());
    assert_eq!(staker_a.shares, Uint128::zero());
    assert_eq!(state.total_shares, staker_a.shares);
}
