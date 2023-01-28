#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::{get_contract_version, set_contract_version};
use localmoney_protocol::errors::ContractError;
use localmoney_protocol::errors::ContractError::HubAlreadyRegistered;
use localmoney_protocol::guards::{
    assert_migration_parameters, assert_multiple_ownership, assert_ownership,
};
use localmoney_protocol::hub_utils::{get_hub_config, register_hub_internal};
use localmoney_protocol::offer::OfferState;
use localmoney_protocol::profile::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, ProfileModel, QueryMsg,
};
use localmoney_protocol::trade::TradeState;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    let res = Response::new().add_attribute("action", "instantiate_profile");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateContact {
            profile_addr,
            contact,
            encryption_key,
        } => update_profile_contact(deps, env, info, profile_addr, contact, encryption_key),
        ExecuteMsg::UpdateTradesCount {
            profile_addr,
            trade_state,
        } => update_trades_count(deps, env, info, profile_addr, trade_state),
        ExecuteMsg::UpdateActiveOffers {
            profile_addr,
            offer_state,
        } => update_active_offers(deps, info, profile_addr, offer_state),
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
    }
}

fn update_profile_contact(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    profile_addr: Addr,
    contact: String,
    encryption_key: String,
) -> Result<Response, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());
    let owners = vec![
        profile_addr.clone(),
        hub_config.trade_addr,
        hub_config.offer_addr,
    ];
    // Only the trade/offer contract or the profile owner should be able to update profile
    assert_multiple_ownership(info.sender, owners).unwrap();

    let storage = deps.storage;
    let mut profile = ProfileModel::query_profile(storage, profile_addr.clone());
    if profile.created_at.eq(&0) {
        let created_at = env.block.time.seconds();
        profile.created_at = created_at
    }
    profile.contact = Some(contact.clone());
    profile.encryption_key = Some(encryption_key.clone());
    ProfileModel::store(storage, &profile);

    let res = Response::new()
        .add_attribute("action", "update_profile")
        .add_attribute("profile_addr", profile_addr.to_string())
        .add_attribute("contact", contact.to_string())
        .add_attribute("encryption_pk", encryption_key.to_string());
    Ok(res)
}

pub fn update_trades_count(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    profile_addr: Addr,
    trade_state: TradeState,
) -> Result<Response, ContractError> {
    // Only the trade contract should be able to call this method.
    let hub_config = get_hub_config(deps.as_ref());
    assert_ownership(info.sender, hub_config.trade_addr).unwrap();

    let mut profile_model = ProfileModel::from_store(deps.storage, profile_addr.clone()).unwrap();
    let profile = &mut profile_model.profile;

    match trade_state {
        TradeState::RequestCreated => {
            profile.requested_trades_count += 1;
            if profile.active_trades_count >= hub_config.active_trades_limit {
                return Err(ContractError::ActiveTradesLimitReached {
                    limit: hub_config.active_trades_limit,
                });
            }
        }
        TradeState::RequestAccepted | TradeState::EscrowFunded => {
            if profile.active_trades_count < hub_config.active_trades_limit {
                profile.active_trades_count += 1;
            } else {
                return Err(ContractError::ActiveTradesLimitReached {
                    limit: hub_config.active_trades_limit,
                });
            }
        }
        TradeState::EscrowCanceled
        | TradeState::EscrowRefunded
        | TradeState::SettledForMaker
        | TradeState::SettledForTaker => {
            // decrease active trades when finished
            if profile.active_trades_count > 0 {
                profile.active_trades_count -= 1;
            }
        }
        TradeState::EscrowReleased => {
            profile.released_trades_count += 1;
            // decrease active trades when finished
            if profile.active_trades_count > 0 {
                profile.active_trades_count -= 1;
            }
        }
        _ => {}
    }
    profile_model.profile.last_trade = env.block.time.seconds();
    let profile = profile_model.save();

    let res = Response::new()
        .add_attribute("action", "increment_trades_count")
        .add_attribute("trade_state", trade_state.to_string())
        .add_attribute(
            "requested_trades_count",
            profile.requested_trades_count.to_string(),
        )
        .add_attribute(
            "released_trades_count",
            profile.released_trades_count.to_string(),
        );
    Ok(res)
}

pub fn update_active_offers(
    deps: DepsMut,
    info: MessageInfo,
    profile_addr: Addr,
    offer_state: OfferState,
) -> Result<Response, ContractError> {
    // Only the Offer contract should be able to call this method.
    let hub_config = get_hub_config(deps.as_ref());
    assert_ownership(info.sender, hub_config.offer_addr)?;

    let mut profile_model = ProfileModel::from_store(deps.storage, profile_addr.clone()).unwrap();
    let profile = &mut profile_model.profile;

    match offer_state {
        OfferState::Active => {
            if profile.active_offers_count < hub_config.active_offers_limit {
                profile.active_offers_count += 1;
            } else {
                return Err(ContractError::ActiveOffersLimitReached {
                    limit: hub_config.active_offers_limit,
                });
            }
        }
        OfferState::Paused => {
            if profile.active_offers_count > 0 {
                profile.active_offers_count -= 1;
            }
        }
        OfferState::Archive => {
            if profile.active_offers_count > 0 {
                profile.active_offers_count -= 1;
            }
        }
    }
    profile_model.save();
    Ok(Response::default())
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Profile { addr } => {
            to_binary(&ProfileModel::query_profile(deps.storage, addr.clone()))
        }
        QueryMsg::Profiles { limit, start_at } => {
            to_binary(&ProfileModel::query_profiles(deps, env, limit, start_at)?)
        }
    }
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
