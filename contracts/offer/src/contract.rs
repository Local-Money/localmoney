use cosmwasm_std::{
    from_binary, to_binary, Api, Binary, Empty, Env, Extern, HandleResponse, HumanAddr,
    InitResponse, Querier, StdError, StdResult, Storage, Uint128,
};

use crate::msg::{ConfigResponse, CreateOfferMsg, HandleMsg, InitMsg, QueryMsg};
use crate::state::{
    config, config_read, query_all_offers, FiatCurrency, Offer, OfferState, OfferType, State,
    OFFERS_KEY,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_storage::{bucket, bucket_read};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State { offers_count: 0 };
    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Create { offer } => try_create_offer(deps, env, offer),
        HandleMsg::Activate { id } => try_activate(deps, env, id),
        HandleMsg::Pause { id } => try_pause(deps, env, id),
    }
}

pub fn try_create_offer<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: CreateOfferMsg,
) -> StdResult<HandleResponse> {
    let mut state = config(&mut deps.storage).load().unwrap();
    let offer_id = state.offers_count + 1;
    state.offers_count = offer_id;

    let offer = Offer {
        id: offer_id,
        owner: env.message.sender,
        offer_type: msg.offer_type,
        fiat_currency: msg.fiat_currency.clone(),
        min_amount: Uint128::from(msg.min_amount),
        max_amount: Uint128::from(msg.max_amount),
        state: OfferState::Active,
    };

    bucket(OFFERS_KEY, &mut deps.storage).save(&offer_id.to_be_bytes(), &offer)?;
    config(&mut deps.storage).save(&state)?;

    Ok(HandleResponse::default())
}

pub fn try_activate<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    env: Env,
    id: u64,
) -> StdResult<HandleResponse> {
    let mut offer = load_offer_by_id(&deps, id)?;
    return if offer.owner.eq(&env.message.sender) {
        if offer.state == OfferState::Paused {
            offer.state = OfferState::Active;
            save_offer(&mut deps, offer)
        } else {
            Err(StdError::generic_err("Offer is Active already."))
        }
    } else {
        Err(StdError::unauthorized())
    };
}

pub fn try_pause<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    env: Env,
    id: u64,
) -> StdResult<HandleResponse> {
    let mut offer = load_offer_by_id(&deps, id)?;
    return if offer.owner.eq(&env.message.sender) {
        if offer.state == OfferState::Active {
            offer.state = OfferState::Paused;
            save_offer(&mut deps, offer)
        } else {
            Err(StdError::generic_err("Offer is not Active."))
        }
    } else {
        Err(StdError::unauthorized())
    };
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::LoadOffers { fiat_currency } => to_binary(&load_offers(deps, fiat_currency)?),
        QueryMsg::LoadOffer { id } => to_binary(&load_offer_by_id(deps, id)?),
    }
}

fn query_config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<State> {
    let state = config_read(&deps.storage).load().unwrap();
    Ok(state)
}

fn load_offers<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    fiat_currency: FiatCurrency,
) -> StdResult<Vec<Offer>> {
    let offers = query_all_offers(deps, fiat_currency)?;
    Ok(offers)
}

fn save_offer<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    offer: Offer,
) -> StdResult<HandleResponse<Empty>> {
    bucket(OFFERS_KEY, &mut deps.storage).save(&offer.id.to_be_bytes(), &offer)?;
    Ok(HandleResponse::default())
}

fn load_offer_by_id<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    id: u64,
) -> StdResult<Offer> {
    let offer: Offer = bucket_read(OFFERS_KEY, &deps.storage)
        .load(&id.to_be_bytes())
        .unwrap();
    Ok(offer)
}

//Tests
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
    let res = create_offer(&mut deps, env.clone(), OfferType::Buy, FiatCurrency::Brl);

    assert_eq!(res.messages.len(), 0);

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse = from_binary(&query(&deps, query_config).unwrap()).unwrap();

    let expected = ConfigResponse { offers_count: 1 };
    assert_eq!(conf, expected);

    let query_cop_offers = QueryMsg::LoadOffers {
        fiat_currency: FiatCurrency::Cop,
    };
    let cop_offers: Vec<Offer> = from_binary(&query(&deps, query_cop_offers).unwrap()).unwrap();
    assert_eq!(cop_offers.len(), 0);

    let query_brl_offers = QueryMsg::LoadOffers {
        fiat_currency: FiatCurrency::Brl,
    };
    let brl_offers: Vec<Offer> = from_binary(&query(&deps, query_brl_offers).unwrap()).unwrap();
    assert_eq!(brl_offers.len(), 1);

    let query_order_by_id = QueryMsg::LoadOffer { id: 1 };
    let created_offer = Offer {
        id: 1,
        owner,
        offer_type: OfferType::Buy,
        fiat_currency: FiatCurrency::Brl,
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
    let res = create_offer(&mut deps, env.clone(), OfferType::Buy, FiatCurrency::Brl);
    assert_eq!(res.messages.len(), 0);

    //Load all offers and get the created offer
    let offers = load_offers(&deps, FiatCurrency::Brl).unwrap();
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
    let res = create_offer(&mut deps, env.clone(), OfferType::Buy, FiatCurrency::Brl);
    assert_eq!(res.messages.len(), 0);

    //Load all offers and get the created offer
    let offers = load_offers(&deps, FiatCurrency::Brl).unwrap();
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
