use cosmwasm_std::{
    to_binary, Api, Binary, Empty, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, Uint128,
};

use crate::msg::{CreateOfferMsg, HandleMsg, InitMsg, QueryMsg};
use crate::state::{
    config, config_read, query_all_offers, Offer, OfferState, State, OFFERS_KEY,
};
use cosmwasm_storage::{bucket, bucket_read};
use crate::currencies::FiatCurrency;

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

pub fn load_offers<S: Storage, A: Api, Q: Querier>(
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

pub fn load_offer_by_id<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    id: u64,
) -> StdResult<Offer> {
    let offer: Offer = bucket_read(OFFERS_KEY, &deps.storage)
        .load(&id.to_be_bytes())
        .unwrap();
    Ok(offer)
}
