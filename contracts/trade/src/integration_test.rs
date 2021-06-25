#![cfg(test)]

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, TradeState};
use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{coin, Addr, BlockInfo, Coin, Empty, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, SimpleBank};
use offer::msg::CreateOfferMsg;
use offer::state::{Offer, OfferType};
use serde::de::DeserializeOwned;

fn mock_app() -> App {
    let env = mock_env();
    let api = Box::new(MockApi::default());
    let bank = SimpleBank {};

    App::new(api, env.block, bank, || Box::new(MockStorage::new()))
}

pub fn offer_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        offer::contract::execute,
        offer::contract::instantiate,
        offer::contract::query,
    );
    Box::new(contract)
}

pub fn trade_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

struct Vars {
    router: App,
    trade_code_id: u64,
    trade_owner: Addr,
    offer_owner: Addr,
    offer_contract_addr: Addr,
    ust_denom: &'static str,
}

fn init() -> Vars {
    let mut router = mock_app();
    //Instantiate Contracts Code
    let offer_code_id = router.store_code(offer_contract());
    let trade_code_id = router.store_code(trade_contract());
    let trade_owner = Addr::unchecked("trade_owner");
    let offer_owner = Addr::unchecked("offer_owner");
    let ust_denom = "uusd";

    //Instantiate Offer contract
    let offer_contract_addr = router
        .instantiate_contract(
            offer_code_id,
            offer_owner.clone(),
            &offer::msg::InstantiateMsg {},
            &[],
            "OFFER",
        )
        .unwrap();

    let create_offer_result = router.execute_contract(
        offer_owner.clone(),
        offer_contract_addr.clone(),
        &offer::msg::ExecuteMsg::Create {
            offer: CreateOfferMsg {
                offer_type: OfferType::Buy,
                fiat_currency: offer::currencies::FiatCurrency::COP,
                min_amount: 1_000,
                max_amount: 500_000,
            },
        },
        &[],
    );
    assert!(create_offer_result.is_ok());

    return Vars {
        router,
        trade_code_id,
        trade_owner,
        offer_owner,
        offer_contract_addr,
        ust_denom,
    };
}

fn create_trade(vars: &mut Vars, trade_amount: Uint128) -> Result<Addr, String> {
    return create_trade_with_funds(vars, trade_amount, true);
}

fn create_trade_with_funds(
    vars: &mut Vars,
    trade_amount: Uint128,
    send_funds: bool,
) -> Result<Addr, String> {
    let router = &mut vars.router;

    let instantiate_trade_msg = InstantiateMsg {
        offer_contract: vars.offer_contract_addr.clone(),
        offer: 1,
        amount: trade_amount.u128() as u64,
    };

    return if send_funds {
        router.instantiate_contract(
            vars.trade_code_id,
            vars.trade_owner.clone(),
            &instantiate_trade_msg,
            &[Coin {
                denom: vars.ust_denom.to_string(),
                amount: trade_amount,
            }],
            "TRADE",
        )
    } else {
        router.instantiate_contract(
            vars.trade_code_id,
            vars.trade_owner.clone(),
            &instantiate_trade_msg,
            &[],
            "TRADE",
        )
    };
}

fn query_offer_contract<T: DeserializeOwned>(
    vars: &mut Vars,
    query_msg: &offer::msg::QueryMsg,
) -> T {
    return vars
        .router
        .wrap()
        .query_wasm_smart(vars.offer_contract_addr.clone(), query_msg)
        .unwrap();
}

fn query_trade_contract<T: DeserializeOwned>(
    vars: &mut Vars,
    trade_contract_addr: Addr,
    query_msg: &QueryMsg,
) -> T {
    return vars
        .router
        .wrap()
        .query_wasm_smart(trade_contract_addr.clone(), query_msg)
        .unwrap();
}

fn query_ust_balance(vars: &mut Vars, addr: Addr) -> Uint128 {
    return vars
        .router
        .wrap()
        .query_balance(addr.clone(), vars.ust_denom.clone())
        .unwrap()
        .amount;
}

#[test]
fn trade_happy_path() {
    let vars = &mut init();

    let initial_trade_owner_balance = Uint128(2_000_000);

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);

    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    //Create Trade
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());
    let trade_contract_addr = create_trade_result.unwrap();

    //Query Trade contract balance (escrow)
    let trade_contract_balance = vars
        .router
        .wrap()
        .query_balance(trade_contract_addr.clone(), vars.ust_denom.clone())
        .unwrap();
    assert_eq!(
        trade_contract_balance,
        Coin {
            denom: vars.ust_denom.to_string(),
            amount: trade_amount
        }
    );

    //Query Trade state and Verify if it's "funded".
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::EscrowFunded);

    let release_response = vars.router.execute_contract(
        vars.trade_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Release {},
        &[],
    );
    assert!(release_response.is_ok());

    //Query Trade state
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});

    let trade_contract_balance = query_ust_balance(vars, trade_contract_addr.clone());
    let trade_owner_balance = query_ust_balance(vars, vars.trade_owner.clone());
    let offer_owner_balance = query_ust_balance(vars, vars.offer_owner.clone());

    assert_eq!(trade_state.state, TradeState::Closed);
    assert_eq!(trade_contract_balance, Uint128::zero());
    assert_eq!(
        trade_owner_balance,
        initial_trade_owner_balance
            .checked_sub(trade_amount)
            .unwrap()
    );
    assert_eq!(offer_owner_balance, trade_amount);
}

#[test]
fn test_trade_amount() {
    let vars = &mut init();

    let initial_trade_owner_balance = Uint128(2_000_000);
    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });

    //Create Trade with amount equal to the offer's min amount
    let mut trade_amount = offer_state.min_amount;
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());

    //Create Trade with amount equal to the offer's max amount
    trade_amount = offer_state.max_amount;
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());

    //Create Trade with amount smaller than the offer's min amount
    trade_amount = Uint128::from(offer_state.min_amount.u128() - 1u128);
    let create_trade_result = create_trade(vars, trade_amount);
    assert_eq!(create_trade_result.is_err(), true);

    //Create Trade with amount bigger than the offer's max amount
    let trade_amount = Uint128::from(offer_state.max_amount.u128() + 1u128);
    let create_trade_result = create_trade(vars, trade_amount);
    assert_eq!(create_trade_result.clone().is_err(), true);
}

#[test]
fn test_trade_expiration() {
    let router = mock_app();
    let from = &router.block_info();
    let vars = &mut init();

    let initial_trade_owner_balance = Uint128(2_000_000);

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);

    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    //Create Trade
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());
    let trade_contract_addr = create_trade_result.unwrap();
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});

    let block_info = BlockInfo {
        height: trade_state.expire_height + 1,
        time: from.time,
        chain_id: from.chain_id.clone(),
    };
    vars.router.set_block(block_info);

    let release_response = vars.router.execute_contract(
        vars.trade_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Release {},
        &[],
    );
    assert!(release_response.is_err());

    let old_trade_contract_balance = query_ust_balance(vars, trade_contract_addr.clone());
    assert_ne!(old_trade_contract_balance, Uint128::zero());

    let old_trade_owner_balance = query_ust_balance(vars, vars.trade_owner.clone());

    let refund_response = vars.router.execute_contract(
        vars.trade_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Refund {},
        &[],
    );
    assert!(refund_response.is_ok());

    let trade_contract_balance = query_ust_balance(vars, trade_contract_addr.clone());
    assert_eq!(trade_contract_balance, Uint128::zero());

    let new_trader_owner_balance = query_ust_balance(vars, vars.trade_owner.clone());
    assert_eq!(
        new_trader_owner_balance,
        old_trade_contract_balance + old_trade_owner_balance,
    );
}

#[test]
fn test_errors() {
    let vars = &mut init();
    let initial_trade_owner_balance = Uint128(2_000_000);

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);

    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    //Create Trade
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());

    //Assert that the state is EscrowFunded
    let trade_contract_addr = create_trade_result.unwrap();
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::EscrowFunded);

    //Try to release as the offer (and not the trade) owner.
    let release_response = vars.router.execute_contract(
        vars.offer_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Release {},
        &[],
    );
    assert!(release_response.is_err());
    assert_eq!(release_response.err().unwrap(), "Unauthorized.");
}

#[test]
fn test_fund_escrow_msg() {
    let vars = &mut init();
    let initial_trade_owner_balance = Uint128(2_000_000);

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);

    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    //Create Trade
    let create_trade_result = create_trade_with_funds(vars, trade_amount, false);
    assert!(create_trade_result.is_ok());

    //Assert that the Trade state is Created
    let trade_contract_addr = create_trade_result.unwrap();
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::Created);

    let send_result = vars.router.execute_contract(
        vars.trade_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::FundEscrow {},
        &[Coin {
            denom: vars.ust_denom.to_string(),
            amount: offer_state.max_amount,
        }],
    );
    assert!(send_result.is_ok());

    //Assert that the Trade state is EscrowFunded
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::EscrowFunded);
}
