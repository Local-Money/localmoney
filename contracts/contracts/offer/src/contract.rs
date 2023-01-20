use crate::state::{offers_count_read, offers_count_storage};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg,
};
use cw2::{get_contract_version, set_contract_version};
use localmoney_protocol::errors::ContractError;
use localmoney_protocol::errors::ContractError::HubAlreadyRegistered;
use localmoney_protocol::guards::{
    assert_migration_parameters, assert_min_g_max, assert_offer_description_valid, assert_ownership,
};
use localmoney_protocol::hub_utils::{get_hub_config, register_hub_internal};
use localmoney_protocol::offer::{
    offers, ExecuteMsg, InstantiateMsg, MigrateMsg, Offer, OfferModel, OfferMsg, OfferResponse,
    OfferState, OfferUpdateMsg, OffersCount, QueryMsg,
};
use localmoney_protocol::profile::{
    load_profile, update_profile_active_offers_msg, update_profile_contact_msg,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();

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
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Offer { id } => to_binary(&load_offer_by_id(deps, id)?),
        QueryMsg::OffersBy {
            offer_type,
            fiat_currency,
            denom,
            order,
            limit,
            last,
        } => to_binary(&OfferModel::query_by(
            deps,
            offer_type,
            fiat_currency,
            denom,
            order,
            limit,
            last,
        )?),
        QueryMsg::OffersByOwner { owner, limit, last } => {
            to_binary(&OfferModel::query_by_owner(deps, owner, limit, last)?)
        }
    }
}

/// Creates a new offer
pub fn create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: OfferMsg,
) -> Result<Response, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());
    assert_min_g_max(msg.min_amount, msg.max_amount)?;

    assert_offer_description_valid(msg.description.clone()).unwrap();

    // Load offers count to create the next sequential id, maybe we can switch to a hash based id in the future.
    let mut offers_count = offers_count_storage(deps.storage)
        .load()
        .unwrap_or(OffersCount { count: 0 });
    offers_count.count += 1;
    let offer_id = offers_count.count;

    // Update profile contact info
    let update_profile_contact_msg = update_profile_contact_msg(
        hub_config.profile_addr.to_string(),
        info.sender.clone(),
        msg.owner_contact.clone(),
        msg.owner_encryption_key.clone(),
    );

    let offer = OfferModel::create(
        deps.storage,
        Offer {
            id: offer_id,
            owner: info.sender.clone(),
            offer_type: msg.offer_type,
            fiat_currency: msg.fiat_currency.clone(),
            rate: msg.rate,
            denom: msg.denom,
            min_amount: msg.min_amount,
            max_amount: msg.max_amount,
            state: OfferState::Active,
            description: msg.description,
            timestamp: env.block.time.seconds(),
        },
    )
    .offer;

    // Update offers count
    offers_count_storage(deps.storage)
        .save(&offers_count)
        .unwrap();

    // Update profile active offers
    let update_profile_offers_msg = update_profile_active_offers_msg(
        hub_config.profile_addr.to_string(),
        info.sender.clone(),
        offer.state,
    );

    let res = Response::new()
        .add_submessage(update_profile_contact_msg)
        .add_submessage(update_profile_offers_msg)
        .add_attribute("action", "create_offer")
        .add_attribute("type", offer.offer_type.to_string())
        .add_attribute("id", offer.id.to_string())
        .add_attribute("rate", offer.rate.to_string())
        .add_attribute("min_amount", offer.min_amount.to_string())
        .add_attribute("max_amount", offer.max_amount.to_string())
        .add_attribute("owner", offer.owner);
    Ok(res)
}

pub fn update_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OfferUpdateMsg,
) -> Result<Response, ContractError> {
    assert_min_g_max(msg.min_amount, msg.max_amount)?;

    let hub_config = get_hub_config(deps.as_ref());
    let mut offer_model = OfferModel::may_load(deps.storage, msg.id);

    assert_ownership(info.sender.clone(), offer_model.offer.owner.clone())?;

    assert_offer_description_valid(msg.description.clone()).unwrap();

    let mut sub_msgs: Vec<SubMsg> = Vec::new();
    if msg.owner_contact.is_some() && msg.owner_encryption_key.is_some() {
        sub_msgs.push(update_profile_contact_msg(
            hub_config.profile_addr.to_string(),
            info.sender.clone(),
            msg.owner_contact.clone().unwrap(),
            msg.owner_encryption_key.clone().unwrap(),
        ));
    }
    if msg.state != offer_model.offer.state {
        sub_msgs.push(update_profile_active_offers_msg(
            hub_config.profile_addr.to_string(),
            info.sender.clone(),
            msg.state.clone(),
        ))
    }

    let offer = offer_model.update(msg);

    let res = Response::new()
        .add_submessages(sub_msgs)
        .add_attribute("action", "update_offer")
        .add_attribute("id", offer.id.to_string())
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

pub fn load_offer_by_id(deps: Deps, id: u64) -> StdResult<OfferResponse> {
    let hub_config = get_hub_config(deps);
    let offer = offers()
        .may_load(deps.storage, id)
        .unwrap_or_default()
        .unwrap();
    let profile = load_profile(
        &deps.querier,
        hub_config.profile_addr.to_string(),
        offer.owner.clone(),
    )
    .unwrap();
    Ok(OfferResponse { offer, profile })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let previous_contract_version = get_contract_version(deps.storage).unwrap();

    assert_migration_parameters(
        previous_contract_version.clone(),
        CONTRACT_NAME.to_string(),
        CONTRACT_VERSION,
    )
    .unwrap();

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    // If the structure of the data in storage changes, we must treat it here

    Ok(Response::default()
        .add_attribute("previous_version", previous_contract_version.version)
        .add_attribute("new_version", CONTRACT_VERSION)
        .add_attribute("name", CONTRACT_NAME))
}
