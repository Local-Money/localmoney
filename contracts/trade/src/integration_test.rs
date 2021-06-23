#![cfg(test)]

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, TradeState};
use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{coin, Addr, Coin, Empty, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, SimpleBank};
use offer;
use offer::msg::CreateOfferMsg;
use offer::state::OfferType;

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

//TODO: use MockQuerier with cw-multi-test to send ust balance changes and side effects/state updates by balance changes.
#[test]
fn trade_happy_path() {
    let mut router = mock_app();

    //Instantiate Contracts Code
    let offer_code_id = router.store_code(offer_contract());
    let trade_code_id = router.store_code(trade_contract());
    let trade_owner = Addr::unchecked("trade_owner");
    let offer_owner = Addr::unchecked("offer_owner");

    let init_funds = [coin(2_000_000, "uusd")].to_vec();
    router.set_bank_balance(&trade_owner, init_funds).unwrap();

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

    //Create offer
    let create_offer_result = router.execute_contract(
        offer_owner.clone(),
        offer_contract_addr.clone(),
        &offer::msg::ExecuteMsg::Create {
            offer: CreateOfferMsg {
                offer_type: OfferType::Buy,
                fiat_currency: offer::currencies::FiatCurrency::COP,
                min_amount: 1_000,
                max_amount: 1_000_000,
            },
        },
        &[],
    );
    assert!(create_offer_result.is_ok());

    let mut instantiate_trade_msg = InstantiateMsg {
        offer_contract: offer_contract_addr.clone(),
        offer: 1,
        amount: 1_500_000,
    };

    //Create trade with an amount bigger than allowed
    let create_trade_response = router.instantiate_contract(
        trade_code_id,
        trade_owner.clone(),
        &instantiate_trade_msg,
        &[],
        "TRADE",
    );
    assert!(create_trade_response.is_err());

    //Fix the amount and try to create it again
    instantiate_trade_msg.amount = 1_000_000;
    //Create Trade
    let trade_contract_addr = router
        .instantiate_contract(
            trade_code_id,
            trade_owner.clone(),
            &instantiate_trade_msg,
            &[],
            "TRADE",
        )
        .unwrap();

    //Release funds
    let _release_response = router
        .execute_contract(
            trade_owner.clone(),
            trade_contract_addr.clone(),
            &ExecuteMsg::Release {},
            &[Coin {
                denom: "uusd".to_string(),
                amount: Uint128(1_000_000),
            }],
        )
        .unwrap();

    //Query Trade state
    let trade_state: State = router
        .wrap()
        .query_wasm_smart(trade_contract_addr.clone(), &QueryMsg::Config {})
        .unwrap();

    //Query Trade contract balance (escrow)
    let _trade_contract_balance = router
        .wrap()
        .query_balance(trade_contract_addr.clone(), "uusd")
        .unwrap();

    //Query trade owner balance
    let _trade_owner_balance = router
        .wrap()
        .query_balance(trade_owner.clone(), "uusd")
        .unwrap();

    //Query trade owner balance
    let _offer_owner_balance = router
        .wrap()
        .query_balance(offer_owner.clone(), "uusd")
        .unwrap();

    assert_eq!(trade_state.state, TradeState::Closed);
}
