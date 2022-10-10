use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, SubMsg, WasmMsg,
};

use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::{HubAlreadyRegistered, Unauthorized};
use localterra_protocol::guards::{assert_min_g_max, assert_ownership};
use localterra_protocol::hub_utils::{get_hub_config, register_hub_internal};
use localterra_protocol::offer::{
    offers, ExecuteMsg, InstantiateMsg, MigrateMsg, Offer, OfferModel, OfferMsg, OfferResponse,
    OfferState, OfferUpdateMsg, OffersCount, QueryMsg,
};
use localterra_protocol::profile::load_profile;
use localterra_protocol::profile::ExecuteMsg as ProfileExecuteMsg;

use crate::state::{offers_count_read, offers_count_storage};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    offers_count_storage(deps.storage)
        .save(&OffersCount { count: 0 })
        .unwrap();

    let res = Response::new().add_attribute("action", "instantiate_offer");
    Ok(res)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
        ExecuteMsg::Create { offer } => create_offer(deps, env, info, offer),
        ExecuteMsg::UpdateOffer { offer_update } => update_offer(deps, env, info, offer_update),
        ExecuteMsg::UpdateLastTraded { offer_id } => update_last_traded(deps, env, info, offer_id),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Offer { id } => to_binary(&load_offer_by_id(deps, id)?),
        QueryMsg::Offers { start_at } => to_binary(&OfferModel::query(deps, start_at)?),
        QueryMsg::OffersBy {
            offer_type,
            fiat_currency,
            denom,
            min,
            max,
            limit,
        } => to_binary(&OfferModel::query_by(
            deps,
            offer_type,
            fiat_currency,
            denom,
            min,
            max,
            limit,
        )?),
        QueryMsg::OffersByOwner { owner, limit } => {
            to_binary(&OfferModel::query_by_owner(deps, owner, limit)?)
        }
    }
}

pub fn create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: OfferMsg,
) -> Result<Response, ContractError> {
    assert_min_g_max(msg.min_amount, msg.max_amount)?;

    let mut offers_count = offers_count_storage(deps.storage).load().unwrap();
    offers_count.count += 1;
    let offer_id = [msg.rate.clone().to_string(), offers_count.count.to_string()].join("_");

    let offer = OfferModel::create(
        deps.storage,
        Offer {
            id: offer_id,
            owner: info.sender.clone(),
            owner_contact: msg.owner_contact,
            offer_type: msg.offer_type,
            fiat_currency: msg.fiat_currency.clone(),
            rate: msg.rate,
            denom: msg.denom,
            min_amount: msg.min_amount,
            max_amount: msg.max_amount,
            state: OfferState::Active,
            timestamp: env.block.time.seconds(),
            last_traded_at: 0,
            trades_count: 0,
        },
    )
    .offer;

    offers_count_storage(deps.storage)
        .save(&offers_count)
        .unwrap();

    let mut res = Response::new()
        .add_attribute("action", "create_offer")
        .add_attribute("type", offer.offer_type.to_string())
        .add_attribute("id", offer.id.to_string())
        .add_attribute("rate", offer.rate.to_string())
        .add_attribute("min_amount", offer.min_amount.to_string())
        .add_attribute("max_amount", offer.max_amount.to_string())
        .add_attribute("owner", offer.owner);

    // Check if profile exists or create one.
    let hub_cfg = get_hub_config(deps.as_ref());
    let profile_result = load_profile(
        &deps.querier,
        info.sender.clone(),
        hub_cfg.profile_addr.to_string(),
    );
    if profile_result.is_err() {
        res = res.add_attribute("create_new_profile", "true");
        res = res.add_submessage(SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: hub_cfg.profile_addr.to_string(),
            msg: to_binary(&ProfileExecuteMsg::Create {
                addr: info.sender.clone(),
            })
            .unwrap(),
            funds: vec![],
        })))
    } else {
        res = res.add_attribute(
            "profile_created_at",
            profile_result.unwrap().created_at.to_string(),
        )
    }

    Ok(res)
}

pub fn update_last_traded(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    offer_id: String,
) -> Result<Response, ContractError> {
    let hub_cfg = get_hub_config(deps.as_ref());

    // Only allows to execute_update_last_traded if called by trade
    if info.sender.ne(&hub_cfg.trade_addr) {
        return Err(Unauthorized {
            owner: hub_cfg.trade_addr,
            caller: info.sender.clone(),
        });
    }

    let mut offer_model = OfferModel::may_load(deps.storage, &offer_id);

    let offer = offer_model.update_last_traded(env.block.time.seconds());

    let res = Response::new()
        .add_attribute("action", "update_last_traded")
        .add_attribute("tradeAddr", info.sender)
        .add_attribute("offer_id", &offer.id)
        .add_attribute("last_traded_at", &offer.last_traded_at.to_string());

    Ok(res)
}

pub fn update_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OfferUpdateMsg,
) -> Result<Response, ContractError> {
    assert_min_g_max(msg.min_amount, msg.max_amount)?;

    let mut offer_model = OfferModel::may_load(deps.storage, &msg.id);

    assert_ownership(info.sender, offer_model.offer.owner.clone())?;

    let offer = offer_model.update(msg);

    let res = Response::new()
        .add_attribute("action", "update_offer")
        .add_attribute("id", offer.id.clone())
        .add_attribute("owner", offer.owner.to_string());

    Ok(res)
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

fn query_state(deps: Deps) -> StdResult<OffersCount> {
    let state = offers_count_read(deps.storage).load().unwrap();
    Ok(state)
}

pub fn load_offer_by_id(deps: Deps, id: String) -> StdResult<OfferResponse> {
    let hub_config = get_hub_config(deps);
    let offer = offers()
        .may_load(deps.storage, id.to_string())
        .unwrap_or_default()
        .unwrap();
    let profile = load_profile(
        &deps.querier,
        offer.clone().owner,
        hub_config.profile_addr.to_string(),
    )
    .unwrap();
    Ok(OfferResponse { offer, profile })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
