#![cfg(test)]
use crate::contract::{execute, instantiate, load_offer_by_id, load_offers, query};
use crate::errors::OfferError;
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{
    from_binary, to_binary, Addr, CosmosMsg, DepsMut, Empty, Env, MessageInfo, ReplyOn, Response,
    SubMsg, Uint128, WasmMsg,
};
use cosmwasm_vm::testing::mock_info;
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::mock_querier::mock_dependencies;
use localterra_protocol::offer::{
    Config, ExecuteMsg, InstantiateMsg, Offer, OfferMsg, OfferState, OfferType, QueryMsg, State,
};
use localterra_protocol::trade::InstantiateMsg as TradeInstantiateMsg;

fn do_init(deps: DepsMut, env: Env, info: MessageInfo) -> Response<Empty> {
    let init_msg = InstantiateMsg {};
    let res = instantiate(deps, env, info, init_msg).unwrap();

    assert_eq!(res.messages.len(), 0);
    return res;
}

#[test]
fn proper_init() {
    let mut deps = mock_dependencies(&[], None);
    let env = mock_env();
    let info = mock_info("factory", &[]);

    let res = do_init(deps.as_mut(), env.clone(), info);
    assert_eq!(res, Response::default());

    let query_config = QueryMsg::Config {};
    let conf: Config =
        from_binary(&query(deps.as_ref(), env.clone(), query_config).unwrap()).unwrap();
    let expected = Config {
        factory_addr: Addr::unchecked("factory"),
    };
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
        offer: OfferMsg {
            offer_type,
            fiat_currency,
            min_amount: 1,
            max_amount: 2,
        },
    };

    return execute(deps, env, info, msg.clone()).unwrap();
}

#[test]
fn create_offer_test() {
    let mut deps = mock_dependencies(&[], None);
    let env = mock_env();
    let factory = Addr::unchecked("factory");
    let info = mock_info(factory.clone().as_str(), &[]);

    do_init(deps.as_mut(), env.clone(), info.clone());
    let res = create_offer(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        OfferType::Buy,
        FiatCurrency::BRL,
    );

    assert_eq!(res.messages.len(), 0);

    let query_state = QueryMsg::State {};
    let state: State =
        from_binary(&query(deps.as_ref(), env.clone(), query_state).unwrap()).unwrap();

    let expected = State { offers_count: 1 };
    assert_eq!(state, expected);

    let query_cop_offers = QueryMsg::Offers {
        fiat_currency: FiatCurrency::COP,
    };
    let cop_offers: Vec<Offer> =
        from_binary(&query(deps.as_ref(), env.clone(), query_cop_offers).unwrap()).unwrap();
    assert_eq!(cop_offers.len(), 0);

    let query_brl_offers = QueryMsg::Offers {
        fiat_currency: FiatCurrency::BRL,
    };
    let brl_offers: Vec<Offer> =
        from_binary(&query(deps.as_ref(), env.clone(), query_brl_offers).unwrap()).unwrap();
    assert_eq!(brl_offers.len(), 1);

    let query_order_by_id = QueryMsg::Offer { id: 1 };
    let created_offer = Offer {
        id: 1,
        owner: factory,
        offer_type: OfferType::Buy,
        fiat_currency: FiatCurrency::BRL,
        min_amount: Uint128::new(1),
        max_amount: Uint128::new(2),
        state: OfferState::Active,
    };
    let queried_offer: Offer =
        from_binary(&query(deps.as_ref(), env.clone(), query_order_by_id).unwrap()).unwrap();
    assert_eq!(queried_offer, created_offer);
}

#[test]
fn pause_offer_test() {
    let mut deps = mock_dependencies(&[], None);
    let factory = Addr::unchecked("factory");
    let other = Addr::unchecked("other");
    let env = mock_env();
    let info = mock_info(factory.clone().as_str(), &[]);

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
    let offers = load_offers(&deps.storage, FiatCurrency::BRL).unwrap();
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
    let offer = &load_offer_by_id(&deps.storage, offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Active);

    //Try to change state with the Owner
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.messages.len(), 0);
    let offer = &load_offer_by_id(&deps.storage, offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Paused);

    //Try to pause Paused offer
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
    assert_eq!(res.is_err(), true);
}

#[test]
fn activate_offer_test() {
    let mut deps = mock_dependencies(&[], None);
    let factory = Addr::unchecked("factory");
    let other = Addr::unchecked("other");
    let env = mock_env();
    let info = mock_info(factory.clone().as_str(), &[]);

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
    let offers = load_offers(&deps.storage, FiatCurrency::BRL).unwrap();
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
    let offer = &load_offer_by_id(&deps.storage, offer.id).unwrap();
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
    let offer = &load_offer_by_id(&deps.storage, offer.id).unwrap();
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
    let offer = &load_offer_by_id(&deps.storage, offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Active);
}

#[test]
fn update_offer_test() {
    let mut deps = mock_dependencies(&[], None);
    let factory = Addr::unchecked("factory");
    let env = mock_env();
    let info = mock_info(factory.clone().as_str(), &[]);

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

    //Load Created message
    let offer = load_offer_by_id(&deps.storage, 1).unwrap();
    assert_eq!(offer.fiat_currency, FiatCurrency::BRL);
    assert_eq!(offer.offer_type, OfferType::Buy);

    //Prepare Update message
    let offer_msg = OfferMsg {
        offer_type: OfferType::Sell,
        fiat_currency: FiatCurrency::COP,
        min_amount: 1000000,
        max_amount: 5000000,
    };
    let update_offer_msg = ExecuteMsg::Update {
        id: 1,
        offer: offer_msg.clone(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), update_offer_msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    //Load offer and check that it was updated
    let offer = load_offer_by_id(&deps.storage, 1).unwrap();
    assert_eq!(offer.offer_type, offer_msg.offer_type);
    assert_eq!(offer.fiat_currency, offer_msg.fiat_currency);
    assert_eq!(offer.min_amount, Uint128::from(offer_msg.min_amount));
    assert_eq!(offer.max_amount, Uint128::from(offer_msg.max_amount));
}

#[test]
fn instantiate_trade() {
    let mut deps = mock_dependencies(&[], None);
    let factory = Addr::unchecked("factory");
    let env = mock_env();
    let info = mock_info(factory.clone().as_str(), &[]);

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

    let trade_amount = Uint128::new(1000000u128);
    //Send Message to Create Trade
    let new_trade_msg = ExecuteMsg::NewTrade {
        offer_id: 1,
        ust_amount: trade_amount.clone(),
        counterparty: "taker".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), new_trade_msg).unwrap();

    let instantiate_msg = WasmMsg::Instantiate {
        admin: None,
        code_id: 0,
        msg: to_binary(&TradeInstantiateMsg {
            offer_id: 1,
            ust_amount: trade_amount.clone(),
            counterparty: "taker".to_string(),
        })
        .unwrap(),
        funds: vec![],
        label: "new-trade".to_string(),
    };
    let sub_message = SubMsg {
        id: 0,
        msg: CosmosMsg::Wasm(instantiate_msg),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };
    assert_eq!(res.messages[0], sub_message);

    let _trades: Vec<String> = from_binary(
        &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Trades {
                maker: "maker".to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
}
