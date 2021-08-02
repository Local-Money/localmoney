use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Uint128, StdError};

use crate::currencies::FiatCurrency;
use crate::errors::OfferError;
use crate::msg::{OfferMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, query_all_offers, Offer, OfferState, State, OFFERS_KEY};
use cosmwasm_storage::{bucket, bucket_read};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State { offers_count: 0 };
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, OfferError> {
    match msg {
        ExecuteMsg::Create { offer } => try_create_offer(deps, env, info, offer),
        ExecuteMsg::Activate { id } => try_activate(deps, env, info, id),
        ExecuteMsg::Pause { id } => try_pause(deps, env, info, id),
        ExecuteMsg::Update { id, offer } => try_update_offer(deps, env, info, id, offer),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::LoadOffers { fiat_currency } => to_binary(&load_offers(deps, fiat_currency)?),
        QueryMsg::LoadOffer { id } => to_binary(&load_offer_by_id(deps, id)?),
    }
}

pub fn try_create_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OfferMsg,
) -> Result<Response, OfferError> {
    let mut state = config(deps.storage).load().unwrap();
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

    if msg.min_amount >= msg.max_amount {
        let err = OfferError::Std(StdError::generic_err("Min amount must be greater than Max amount."));
        return Err(err)
    }

    bucket(deps.storage, OFFERS_KEY).save(&offer_id.to_be_bytes(), &offer)?;
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

pub fn try_activate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, OfferError> {
    let mut offer = load_offer_by_id(deps.as_ref(), id)?;
    return if offer.owner.eq(&info.sender) {
        if offer.state == OfferState::Paused {
            offer.state = OfferState::Active;
            Ok(save_offer(deps, offer)?)
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

pub fn try_pause(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, OfferError> {
    let mut offer = load_offer_by_id(deps.as_ref(), id)?;
    return if offer.owner.eq(&info.sender) {
        if offer.state == OfferState::Active {
            offer.state = OfferState::Paused;
            Ok(save_offer(deps, offer)?)
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

pub fn try_update_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    msg: OfferMsg,
) -> Result<Response, OfferError> {
    let mut offer = load_offer_by_id(deps.as_ref(), id)?;

    if msg.min_amount >= msg.max_amount {
        let err = OfferError::Std(StdError::generic_err("Min amount must be greater than Max amount."));
        return Err(err)
    }

    return if offer.owner.eq(&info.sender) {
        offer.offer_type = msg.offer_type;
        offer.fiat_currency = msg.fiat_currency;
        offer.min_amount = Uint128::from(msg.min_amount);
        offer.max_amount = Uint128::from(msg.max_amount);
        Ok(save_offer(deps, offer)?)
    } else {
        Err(OfferError::Unauthorized {
            owner: offer.owner,
            caller: info.sender,
        })
    };
}

fn query_config(deps: Deps) -> StdResult<State> {
    let state = config_read(deps.storage).load().unwrap();
    Ok(state)
}

pub fn load_offers(deps: Deps, fiat_currency: FiatCurrency) -> StdResult<Vec<Offer>> {
    let offers = query_all_offers(deps, fiat_currency)?;
    Ok(offers)
}

fn save_offer(deps: DepsMut, offer: Offer) -> StdResult<Response<Empty>> {
    bucket(deps.storage, OFFERS_KEY).save(&offer.id.to_be_bytes(), &offer)?;
    Ok(Response::default())
}

pub fn load_offer_by_id(deps: Deps, id: u64) -> StdResult<Offer> {
    let offer: Offer = bucket_read(deps.storage, OFFERS_KEY)
        .load(&id.to_be_bytes())
        .unwrap();
    Ok(offer)
}
