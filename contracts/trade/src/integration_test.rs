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

#[test]
fn trade_happy_path() {
    let mut router = mock_app();

    //Instantiate Contracts Code
    let offer_code_id = router.store_code(offer_contract());
    let trade_code_id = router.store_code(trade_contract());
    let trade_owner = Addr::unchecked("trade_owner");
    let offer_owner = Addr::unchecked("offer_owner");

    let ust_denom = "uusd";
    let initial_trade_owner_balance = Uint128(2_000_000);
    let trade_amount = Uint128(1_000_000);

    let init_funds = [coin(*&initial_trade_owner_balance.u128(), "uusd")].to_vec();
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
                max_amount: trade_amount.u128() as u64,
            },
        },
        &[],
    );
    assert!(create_offer_result.is_ok());

    let instantiate_trade_msg = InstantiateMsg {
        offer_contract: offer_contract_addr.clone(),
        offer: 1,
        amount: trade_amount.u128() as u64,
    };

    //Create Trade
    let trade_contract_addr = router
        .instantiate_contract(
            trade_code_id,
            trade_owner.clone(),
            &instantiate_trade_msg,
            &[Coin {
                denom: ust_denom.to_string(),
                amount: trade_amount,
            }],
            "TRADE",
        )
        .unwrap();

    //Query Trade contract balance (escrow)
    let trade_contract_balance = router
        .wrap()
        .query_balance(trade_contract_addr.clone(), ust_denom)
        .unwrap();
    assert_eq!(
        trade_contract_balance,
        Coin {
            denom: ust_denom.to_string(),
            amount: trade_amount
        }
    );

    //Query Trade state and Verify if it's "funded".
    let trade_state: State = router
        .wrap()
        .query_wasm_smart(trade_contract_addr.clone(), &QueryMsg::Config {})
        .unwrap();
    assert_eq!(trade_state.state, TradeState::EscrowFunded);

    let release_response = router.execute_contract(
        trade_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Release {},
        &[],
    );
    assert!(release_response.is_ok());

    //Query Trade state
    let trade_state: State = router
        .wrap()
        .query_wasm_smart(trade_contract_addr.clone(), &QueryMsg::Config {})
        .unwrap();

    //Query Trade contract balance (escrow)
    let trade_contract_balance = router
        .wrap()
        .query_balance(trade_contract_addr.clone(), ust_denom)
        .unwrap();

    //Query trade owner balance
    let trade_owner_balance = router
        .wrap()
        .query_balance(trade_owner.clone(), ust_denom)
        .unwrap();

    //Query trade owner balance
    let offer_owner_balance = router
        .wrap()
        .query_balance(offer_owner.clone(), ust_denom)
        .unwrap();

    assert_eq!(trade_state.state, TradeState::Closed);
    assert_eq!(trade_contract_balance.amount, Uint128::zero());
    assert_eq!(trade_owner_balance.amount, trade_amount);
    assert_eq!(offer_owner_balance.amount, trade_amount);
    assert_eq!(offer_owner_balance.denom, ust_denom);
}
