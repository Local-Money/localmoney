#![cfg(test)]

use std::ops::Add;

use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{
    from_binary, Addr, BankMsg, Coin, CosmosMsg, DepsMut, Empty, MessageInfo, OwnedDeps, Response,
    SubMsg, Uint128,
};
use cosmwasm_vm::testing::{mock_env, mock_info};

use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::offer::{Offer, OfferState, OfferType};
use localterra_protocol::trade::{ExecuteMsg, InstantiateMsg, QueryMsg, State, TradeState};

use crate::contract::{execute, instantiate, query, subtract_localterra_fee};
use crate::errors::TradeError;
use crate::mock_querier::{mock_dependencies, WasmMockQuerier};

#[test]
fn test_init() {
    let mut deps = mock_dependencies(&[], None);
    let info = mock_info("taker", &[]);
    let trade_amount = Uint128::new(10_000_000u128);

    let instantiate_trade_msg = InstantiateMsg {
        offer_id: 1,
        ust_amount: trade_amount.clone().to_string(),
        counterparty: "other".to_string(),
        offers_addr: "offers".to_string(),
    };

    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_trade_msg);
    assert!(res.is_ok())
}

fn create_trade(
    trade_amount: Uint128,
    info: MessageInfo,
    offer: Option<Offer>,
) -> (
    Result<Response<Empty>, TradeError>,
    OwnedDeps<MockStorage, MockApi, WasmMockQuerier>,
) {
    let mut deps = mock_dependencies(
        &[Coin {
            denom: "uusd".to_string(),
            amount: trade_amount.clone(),
        }],
        offer,
    );

    //Init trade
    let instantiate_trade_msg = InstantiateMsg {
        offer_id: 1,
        ust_amount: trade_amount.clone().to_string(),
        counterparty: info.sender.clone().into_string(),
        offers_addr: "offers".to_string(),
    };
    let res = instantiate(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        instantiate_trade_msg,
    );
    (res, deps)
}

fn mock_info_with_ust(sender: &str, amount: Uint128) -> MessageInfo {
    mock_info(
        sender,
        &[Coin {
            denom: "uusd".to_string(),
            amount: amount.clone(),
        }],
    )
}

fn release_trade(deps: DepsMut, info: MessageInfo) -> Result<Response<Empty>, TradeError> {
    let res = execute(deps, mock_env(), info.clone(), ExecuteMsg::Release {});
    assert!(&res.is_ok());
    return res;
}

///Test trade full happy path:
/// - Create Trade with funds
/// - Assert that state is EscrowFunded
/// - Send Release msg and assert that state is closed and that Bank::Send msg was sent.
#[test]
fn test_trade_happy_path() {
    let trade_amount = Uint128::from(500_000_000u128);
    //let local_terra_fee = trade_amount.checked_div(Uint128::new(1000)).unwrap();
    //let received_amount = trade_amount.clone() - local_terra_fee.clone();

    let info = mock_info_with_ust("taker", trade_amount);
    let (_, mut deps) = create_trade(trade_amount.clone(), info.clone(), None);

    //Trade should be in funded state
    let trade_state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    assert_eq!(trade_state.state, TradeState::EscrowFunded);

    //Send release message
    let _res = release_trade(deps.as_mut(), info.clone());

    //Check that trade state is Closed
    let trade_state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    assert_eq!(trade_state.state, TradeState::Closed);

    //Verify that the correct messages were sent after trade completion
    /*
    assert_eq!(
        res.unwrap().messages,
        vec![
            // Fee collector subMessage
            /*
            SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                // TODO change the local terra fee collector address
                to_address: "fee-collector".to_string(),
                amount: vec![Coin {
                    denom: "uusd".to_string(),
                    // 1% fee amount
                    amount: local_terra_fee
                }]
            })),
             */
            // Offer owner message
            SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: "offer-owner".to_string(),
                amount: vec![Coin {
                    denom: "uusd".to_string(),
                    // The amount sent has a 1% discount
                    amount: trade_amount.clone()
                }]
            })),
            /*
            // Trading incentives registration message.
            SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "cosmos2contract".to_string(),
                msg: to_binary(&TradingIncentivesMsg::RegisterTrade {
                    trade: "cosmos2contract".to_string(),
                    maker: "offer-owner".to_string()
                })
                .unwrap(),
                funds: vec![]
            }))
             */
        ]
    )
     */
}

fn create_offer_struct(
    min_amount: Uint128,
    max_amount: Uint128,
    offer_type: Option<OfferType>,
    fiat_currency: Option<FiatCurrency>,
) -> Offer {
    Offer {
        id: 1,
        owner: Addr::unchecked("offer-owner"),
        offer_type: offer_type.clone().unwrap_or(OfferType::Buy),
        fiat_currency: fiat_currency.clone().unwrap_or(FiatCurrency::COP),
        min_amount: min_amount.clone(),
        max_amount: max_amount.clone(),
        state: OfferState::Active,
    }
}

///Verifies that a trade can only be created within order amount limits.
#[test]
fn test_trade_amount() {
    //Init trade with min amount and assert that it's ok.
    let min_amount = Uint128::new(1_000_000u128);
    let max_amount = Uint128::new(500_000_000u128);
    let offer = create_offer_struct(min_amount, max_amount, None, None);
    let trade_amount = min_amount.clone();
    let taker = mock_info_with_ust("taker", min_amount.clone());
    let (res, _) = create_trade(trade_amount, taker.clone(), Some(offer.clone()));
    assert!(res.is_ok());

    //Init trade with max amount and assert it's ok.
    let trade_amount = max_amount.clone();
    let taker = mock_info_with_ust("taker", min_amount.clone());
    let (res, _) = create_trade(trade_amount, taker.clone(), Some(offer.clone()));
    assert!(res.is_ok());

    //Init trade with less than min amount and assert it's an err.
    let trade_amount = min_amount.checked_sub(Uint128::new(1u128)).unwrap();
    let taker = mock_info_with_ust("taker", min_amount.clone());
    let (res, _) = create_trade(trade_amount, taker.clone(), Some(offer.clone()));
    assert!(res.is_err());

    //Init trade with more than max amount and assert it's an err.
    let trade_amount = max_amount.add(Uint128::new(1u128));
    let taker = mock_info_with_ust("taker", min_amount.clone());
    let (res, _) = create_trade(trade_amount, taker.clone(), Some(offer.clone()));
    assert!(res.is_err());
}

///Test trade expiration.
#[test]
fn test_trade_expiration() {
    let trade_amount = Uint128::new(500_000_000u128);
    let info = mock_info_with_ust("taker", trade_amount);
    let (_, mut deps) = create_trade(trade_amount, info.clone(), None);

    //Set env.block to trade.expiration_height + 1
    let trade_state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();

    let mut expired_env = mock_env();
    expired_env.block.height = trade_state.expire_height + 1;

    //Send Release Message, assert that it's an error.
    let res = execute(
        deps.as_mut(),
        expired_env.clone(),
        info.clone(),
        ExecuteMsg::Release {},
    );
    assert!(res.is_err());
    assert!(matches!(res.err().unwrap(), TradeError::Expired { .. }));
}

///Test custom error types.
#[test]
fn test_custom_errors() {
    let trade_amount = Uint128::new(500_000_000u128);
    let info = mock_info_with_ust("taker", trade_amount);
    let other_info = mock_info("other", &[]);

    //Create Trade and assert that it's funded.
    let (_, mut deps) = create_trade(trade_amount, info.clone(), None);
    let trade_state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    assert_eq!(trade_state.state, TradeState::EscrowFunded);

    //Try to release Trade with another user.
    let res = execute(
        deps.as_mut(),
        mock_env(),
        other_info,
        ExecuteMsg::Release {},
    );
    assert!(matches!(
        res.err().unwrap(),
        TradeError::Unauthorized { .. }
    ))
}

///Test Refund
#[test]
fn test_refund() {
    let trade_amount = Uint128::new(500_000_000u128);
    let info = mock_info_with_ust("taker", trade_amount);
    let any_info = mock_info("any", &[]);
    let (_, mut deps) = create_trade(trade_amount, info.clone(), None);

    //Try to refund and assert it's an err.
    let res = execute(
        deps.as_mut(),
        mock_env(),
        any_info.clone(),
        ExecuteMsg::Refund {},
    );
    assert!(res.is_err());
    assert!(matches!(res.err().unwrap(), TradeError::RefundError { .. }));

    //Set env.block to trade.expiration_height + 1
    let trade_state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    let mut expired_env = mock_env();
    expired_env.block.height = trade_state.expire_height + 1;

    //Try to refund and assert it's ok.
    let res = execute(
        deps.as_mut(),
        expired_env,
        any_info.clone(),
        ExecuteMsg::Refund {},
    );
    assert!(res.is_ok());
    //Verify that the correct messages were sent after trade completion
    assert_eq!(
        res.unwrap().messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: "uusd".to_string(),
                amount: trade_amount.clone(),
            }],
        }))]
    )
}

///Test fund escrow after instantiating Trade without coins
#[test]
fn test_fund_escrow() {
    let mut trade_amount = Uint128::from(500_000_000u128);
    let mut info = mock_info_with_ust("taker", Uint128::zero());
    let (_, mut deps) = create_trade(trade_amount.clone(), info.clone(), None);

    //Trade should be in Created state
    let trade_state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    assert_eq!(trade_state.state, TradeState::Created);

    //Send FundEscrow message with UST and check that trade is in EscrowFunded state.
    let localterra_fee = subtract_localterra_fee(trade_amount);
    trade_amount = trade_amount.add(localterra_fee);
    info.funds[0].amount = trade_amount.clone();

    let res = execute(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        ExecuteMsg::FundEscrow {},
    );
    assert!(res.is_ok());
    let trade_state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    assert_eq!(trade_state.state, TradeState::EscrowFunded);
}

#[test]
fn test_expired_trade() {
    let mut trade_amount = Uint128::from(500_000_000u128);
    let mut info = mock_info_with_ust("taker", Uint128::zero());
    let (_, mut deps) = create_trade(trade_amount.clone(), info.clone(), None);

    let trade_state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();

    //Send FundEscrow message with UST and check that trade is in EscrowFunded state.
    let localterra_fee = subtract_localterra_fee(trade_amount);
    trade_amount = trade_amount.add(localterra_fee);
    info.funds[0].amount = trade_amount.clone();

    let mut env = mock_env();
    env.block.height = trade_state.expire_height;

    let res = execute(deps.as_mut(), env, info.clone(), ExecuteMsg::FundEscrow {});
    assert!(res.is_err());
    assert!(matches!(res.err().unwrap(), TradeError::Expired { .. }));
}
