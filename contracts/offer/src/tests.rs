use crate::contract::{handle, init, load_offer_by_id, load_offers, query};
use crate::currencies::FiatCurrency;
use crate::msg::{ConfigResponse, CreateOfferMsg, HandleMsg, InitMsg, QueryMsg};
use crate::state::{Offer, OfferState, OfferType};
use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_std::{
    from_binary, Api, Empty, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, Storage, Uint128,
};

fn do_init<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    env: Env,
) -> InitResponse<Empty> {
    let init_msg = InitMsg {};
    let res = init(&mut deps, env, init_msg).unwrap();

    assert_eq!(res.messages.len(), 0);
    return res;
}

#[test]
fn proper_init() {
    let mut deps = mock_dependencies(20, &[]);
    let owner = HumanAddr::from("owner");
    let env = mock_env(owner, &[]);

    let res = do_init(&mut deps, env.clone());
    assert_eq!(res, InitResponse::default());

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse = from_binary(&query(&deps, query_config).unwrap()).unwrap();

    let expected = ConfigResponse { offers_count: 0 };

    assert_eq!(conf, expected);
}

fn create_offer<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    env: Env,
    offer_type: OfferType,
    fiat_currency: FiatCurrency,
) -> HandleResponse<Empty> {
    let msg = HandleMsg::Create {
        offer: CreateOfferMsg {
            offer_type,
            fiat_currency,
            min_amount: 0,
            max_amount: 0,
        },
    };

    return handle(&mut deps, env, msg.clone()).unwrap();
}

#[test]
fn create_offer_test() {
    let mut deps = mock_dependencies(20, &[]);
    let owner = HumanAddr::from("owner");
    let env = mock_env(owner.clone(), &[]);

    do_init(&mut deps, env.clone());
    let res = create_offer(&mut deps, env.clone(), OfferType::Buy, FiatCurrency::BRL);

    assert_eq!(res.messages.len(), 0);

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse = from_binary(&query(&deps, query_config).unwrap()).unwrap();

    let expected = ConfigResponse { offers_count: 1 };
    assert_eq!(conf, expected);

    let query_cop_offers = QueryMsg::LoadOffers {
        fiat_currency: FiatCurrency::COP,
    };
    let cop_offers: Vec<Offer> = from_binary(&query(&deps, query_cop_offers).unwrap()).unwrap();
    assert_eq!(cop_offers.len(), 0);

    let query_brl_offers = QueryMsg::LoadOffers {
        fiat_currency: FiatCurrency::BRL,
    };
    let brl_offers: Vec<Offer> = from_binary(&query(&deps, query_brl_offers).unwrap()).unwrap();
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
    let queried_offer: Offer = from_binary(&query(&deps, query_order_by_id).unwrap()).unwrap();
    assert_eq!(queried_offer, created_offer);
}

#[test]
fn pause_offer_test() {
    let mut deps = mock_dependencies(20, &[]);
    let owner = HumanAddr::from("owner");
    let other = HumanAddr::from("other");
    let env = mock_env(owner.clone(), &[]);

    //Create Offer
    do_init(&mut deps, env.clone());
    let res = create_offer(&mut deps, env.clone(), OfferType::Buy, FiatCurrency::BRL);
    assert_eq!(res.messages.len(), 0);

    //Load all offers and get the created offer
    let offers = load_offers(&deps, FiatCurrency::BRL).unwrap();
    let offer = &offers[0];
    assert_eq!(offer.state, OfferState::Active);

    //Pause Message
    let msg = HandleMsg::Pause { id: offer.id };
    let other_env = mock_env(other.clone(), &[]);

    //Try to change the State with another address.
    let res = handle(&mut deps, other_env.clone(), msg.clone());
    assert_eq!(res.err().unwrap(), StdError::unauthorized());
    let offer = &load_offer_by_id(&deps, offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Active);

    //Try to change state with the Owner
    let res = handle(&mut deps, env.clone(), msg.clone()).unwrap();
    assert_eq!(res.messages.len(), 0);
    let offer = &load_offer_by_id(&deps, offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Paused);

    //Try to pause Paused offer
    let res = handle(&mut deps, env.clone(), msg.clone());
    assert_eq!(res.is_err(), true);
}

#[test]
fn activate_offer_test() {
    let mut deps = mock_dependencies(20, &[]);
    let owner = HumanAddr::from("owner");
    let other = HumanAddr::from("other");
    let env = mock_env(owner.clone(), &[]);

    //Create Offer
    do_init(&mut deps, env.clone());
    let res = create_offer(&mut deps, env.clone(), OfferType::Buy, FiatCurrency::BRL);
    assert_eq!(res.messages.len(), 0);

    //Load all offers and get the created offer
    let offers = load_offers(&deps, FiatCurrency::BRL).unwrap();
    let offer = &offers[0];
    assert_eq!(offer.state, OfferState::Active);

    //Pause Message
    let pause_msg = HandleMsg::Pause { id: offer.id };
    let activate_msg = HandleMsg::Activate { id: offer.id };
    let other_env = mock_env(other.clone(), &[]);

    //Try to change state to Paused with the Owner
    let res = handle(&mut deps, env.clone(), pause_msg.clone()).unwrap();
    assert_eq!(res.messages.len(), 0);
    let offer = &load_offer_by_id(&deps, offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Paused);

    //Try to change the State to Active with another address.
    let res = handle(&mut deps, other_env.clone(), activate_msg.clone());
    assert_eq!(res.err().unwrap(), StdError::unauthorized());
    let offer = &load_offer_by_id(&deps, offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Paused);

    //Try to change state to Active with the Owner
    let res = handle(&mut deps, env.clone(), activate_msg.clone()).unwrap();
    assert_eq!(res.messages.len(), 0);
    let offer = &load_offer_by_id(&deps, offer.id).unwrap();
    assert_eq!(offer.state, OfferState::Active);
}
