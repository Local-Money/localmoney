use cosmwasm_std::{
    to_binary, Api, Binary, Empty, Env, Extern, HandleResponse, InitResponse, MessageInfo, Querier,
    StdResult, Storage, Uint128,
};

use crate::currencies::FiatCurrency;
use crate::errors::OfferError;
use crate::msg::{CreateOfferMsg, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, query_all_offers, Offer, OfferState, State, OFFERS_KEY};
use cosmwasm_storage::{bucket, bucket_read};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State { offers_count: 0 };
    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, OfferError> {
    match msg {
        HandleMsg::Create { offer } => try_create_offer(deps, env, info, offer),
        HandleMsg::Activate { id } => try_activate(deps, env, info, id),
        HandleMsg::Pause { id } => try_pause(deps, env, info, id),
    }
}

pub fn try_create_offer<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    info: MessageInfo,
    msg: CreateOfferMsg,
) -> Result<HandleResponse, OfferError> {
    let mut state = config(&mut deps.storage).load().unwrap();
    let offer_id = state.offers_count + 1;
    state.offers_count = offer_id;

    let offer = Offer {
        id: offer_id,
        owner: info.sender,
        offer_type: msg.offer_type,
        fiat_currency: msg.fiat_currency.clone(),
        min_amount: Uint128::from(msg.min_amount),
        max_amount: Uint128::from(msg.max_amount),
        state: OfferState::Active,
    };

    bucket(&mut deps.storage, OFFERS_KEY).save(&offer_id.to_be_bytes(), &offer)?;
    config(&mut deps.storage).save(&state)?;

    Ok(HandleResponse::default())
}

pub fn try_activate<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<HandleResponse, OfferError> {
    let mut offer = load_offer_by_id(&deps, id)?;
    return if offer.owner.eq(&info.sender) {
        if offer.state == OfferState::Paused {
            offer.state = OfferState::Active;
            Ok(save_offer(&mut deps, offer)?)
        } else {
            Err(OfferError::InvalidStateChange {
                from: offer.state,
                to: OfferState::Active,
            })
        }
    } else {
        Err(OfferError::Unauthorized {
            owner: offer.owner,
            caller: info.sender,
        })
    };
}

pub fn try_pause<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<HandleResponse, OfferError> {
    let mut offer = load_offer_by_id(&deps, id)?;
    return if offer.owner.eq(&info.sender) {
        if offer.state == OfferState::Active {
            offer.state = OfferState::Paused;
            Ok(save_offer(&mut deps, offer)?)
        } else {
            Err(OfferError::InvalidStateChange {
                from: offer.state,
                to: OfferState::Paused,
            })
        }
    } else {
        Err(OfferError::Unauthorized {
            owner: offer.owner,
            caller: info.sender,
        })
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
    bucket(&mut deps.storage, OFFERS_KEY).save(&offer.id.to_be_bytes(), &offer)?;
    Ok(HandleResponse::default())
}

pub fn load_offer_by_id<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    id: u64,
) -> StdResult<Offer> {
    let offer: Offer = bucket_read(&deps.storage, OFFERS_KEY)
        .load(&id.to_be_bytes())
        .unwrap();
    Ok(offer)
}
