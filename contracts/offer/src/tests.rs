#![cfg(test)]
use crate::contract::{execute, instantiate, load_offer_by_id, load_offers, query};
use crate::currencies::FiatCurrency;
use crate::errors::OfferError;
use crate::msg::{ConfigResponse, CreateOfferMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Offer, OfferState, OfferType};
use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_std::{from_binary, Addr, DepsMut, Empty, Env, MessageInfo, Response, Uint128};
use cosmwasm_vm::testing::mock_info;

fn do_init(deps: DepsMut, env: Env, info: MessageInfo) -> Response<Empty> {
    let init_msg = InstantiateMsg {};
    let res = instantiate(deps, env, info, init_msg).unwrap();

    assert_eq!(res.messages.len(), 0);
    return res;
}

#[test]
fn proper_init() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("owner", &[]);

    let res = do_init(deps.as_mut(), env.clone(), info);
    assert_eq!(res, Response::default());

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse =
        from_binary(&query(deps.as_ref(), env.clone(), query_config).unwrap()).unwrap();
    let expected = ConfigResponse { offers_count: 0 };
    assert_eq!(conf, expected);
}

fn create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    offer_type: OfferType,
    fiat_currency: FiatCurrency,
) -> Response<Empty> {
    let msg = ExecuteMsg::Create {
        offer: CreateOfferMsg {
            offer_type,
            fiat_currency,
            min_amount: 0,
            max_amount: 0,
        },
    };

    return execute(deps, env, info, msg.clone()).unwrap();
}

#[test]
fn create_offer_test() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let owner = Addr::unchecked("owner");
    let info = mock_info(owner.clone().as_str(), &[]);

    do_init(deps.as_mut(), env.clone(), info.clone());
    let res = create_offer(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        OfferType::Buy,
        FiatCurrency::BRL,
    );

    assert_eq!(res.messages.len(), 0);

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse =
        from_binary(&query(deps.as_ref(), env.clone(), query_config).unwrap()).unwrap();

    let expected = ConfigResponse { offers_count: 1 };
    assert_eq!(conf, expected);

    let query_cop_offers = QueryMsg::LoadOffers {
        fiat_currency: FiatCurrency::COP,
    };
    let cop_offers: Vec<Offer> =
        from_binary(&query(deps.as_ref(), env.clone(), query_cop_offers).unwrap()).unwrap();
    assert_eq!(cop_offers.len(), 0);

    let query_brl_offers = QueryMsg::LoadOffers {
        fiat_currency: FiatCurrency::BRL,
    };
    let brl_offers: Vec<Offer> =
        from_binary(&query(deps.as_ref(), env.clone(), query_brl_offers).unwrap()).unwrap();
    assert_eq!(brl_offers.len(), 1);

    let query_order_by_id = QueryMsg::LoadOffer { id: 1 };
    let created_offer = Offer {
        id: 1,
        owner,
        offer_type: OfferType::Buy,
        fiat_currency: FiatCurrency::BRL,
        min_amount: Uint128(0),
        max_amount: Uint128(0),
        state: OfferState::Active,
    };
    let queried_offer: Offer =
        from_binary(&query(deps.as_ref(), env.clone(), query_order_by_id).unwrap()).unwrap();
    assert_eq!(queried_offer, created_offer);
}

#[test]
fn pause_offer_test() {
    let mut deps = mock_dependencies(&[]);
    let owner = Addr::unchecked("owner");
    let other = Addr::unchecked("other");
    let env = mock_env();
    let info = mock_info(owner.clone().as_str(), &[]);

    //Create Offer
    do_init(deps.as_mut(), env.clone(), info.clone());
    let res = create_offer(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        OfferType::Buy,
        FiatCurrency::BRL,
    );
    assert_eq!(res.messages.len(), 0);

    //Load all offers and get the created offer
    let offers = load_offers(deps.as_ref(), FiatCurrency::BRL).unwrap();
    let offer = &offers[0];
    assert_eq!(offer.state, OfferState::Active);

    //Pause Message
    let msg = ExecuteMsg::Pause { id: offer.id };
    let other_env = mock_env();
    let other_info = mock_info(other.clone().as_str(), &[]);

    //Try to change the State with another address.
    let res = execute(
        deps.as_mut(),
        other_env.clone(),
        other_info.clone(),
        msg.clone(),
    );
    assert!(matches!(
        res.err().unwrap(),
        OfferError::Unauthorized { .. }
    ));
    let offer = &load_offer_by_id(deps.as_ref(), offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Active);

    //Try to change state with the Owner
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.messages.len(), 0);
    let offer = &load_offer_by_id(deps.as_ref(), offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Paused);

    //Try to pause Paused offer
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
    assert_eq!(res.is_err(), true);
}

#[test]
fn activate_offer_test() {
    let mut deps = mock_dependencies(&[]);
    let owner = Addr::unchecked("owner");
    let other = Addr::unchecked("other");
    let env = mock_env();
    let info = mock_info(owner.clone().as_str(), &[]);

    //Create Offer
    do_init(deps.as_mut(), env.clone(), info.clone());
    let res = create_offer(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        OfferType::Buy,
        FiatCurrency::BRL,
    );
    assert_eq!(res.messages.len(), 0);

    //Load all offers and get the created offer
    let offers = load_offers(deps.as_ref(), FiatCurrency::BRL).unwrap();
    let offer = &offers[0];
    assert_eq!(offer.state, OfferState::Active);

    //Pause Message
    let pause_msg = ExecuteMsg::Pause { id: offer.id };
    let activate_msg = ExecuteMsg::Activate { id: offer.id };
    let other_env = mock_env();
    let other_info = mock_info(other.clone().as_str(), &[]);

    //Try to change state to Paused with the Owner
    let res = execute(deps.as_mut(), env.clone(), info.clone(), pause_msg.clone()).unwrap();
    assert_eq!(res.messages.len(), 0);
    let offer = &load_offer_by_id(deps.as_ref(), offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Paused);

    //Try to change the State to Active with another address.
    let res = execute(
        deps.as_mut(),
        other_env.clone(),
        other_info.clone(),
        activate_msg.clone(),
    );

    assert!(matches!(
        res.err().unwrap(),
        OfferError::Unauthorized { .. }
    ));
    let offer = &load_offer_by_id(deps.as_ref(), offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Paused);

    //Try to change state to Active with the Owner
    let res = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        activate_msg.clone(),
    )
    .unwrap();
    assert_eq!(res.messages.len(), 0);
    let offer = &load_offer_by_id(deps.as_ref(), offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Active);
}
