#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::HubAlreadyRegistered;
use localterra_protocol::guards::{assert_multiple_ownership, assert_ownership};
use localterra_protocol::hub_utils::{get_hub_config, register_hub_internal};
use localterra_protocol::kujira::msg::KujiraMsg;
use localterra_protocol::kujira::query::KujiraQuery;
use localterra_protocol::profile::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, ProfileModel, QueryMsg,
};
use localterra_protocol::trade::TradeState;

// version info for migration info
pub const CONTRACT_NAME: &str = "localmoney.io:profile";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    let res = Response::new().add_attribute("action", "instantiate_profile");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<KujiraQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    match msg {
        ExecuteMsg::UpdateProfile {
            profile_addr,
            contact,
            encryption_key,
        } => update_profile(deps, env, info, profile_addr, contact, encryption_key),
        ExecuteMsg::IncreaseTradeCount {
            profile_addr,
            trade_state,
        } => increase_trades_count(deps, env, info, profile_addr, trade_state),
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
    }
}

fn update_profile(
    deps: DepsMut<KujiraQuery>,
    env: Env,
    info: MessageInfo,
    profile_addr: Addr,
    contact: String,
    encryption_key: String,
) -> Result<Response<KujiraMsg>, ContractError> {
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

pub fn increase_trades_count(
    deps: DepsMut<KujiraQuery>,
    env: Env,
    info: MessageInfo,
    profile_addr: Addr,
    trade_state: TradeState,
) -> Result<Response<KujiraMsg>, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());

    // Only the trade contract should be able to call increase_trades_count
    assert_ownership(info.sender, hub_config.trade_addr).unwrap();

    let profile_result = ProfileModel::from_store(deps.storage, profile_addr.clone());
    let mut profile_model = profile_result.unwrap();
    match trade_state {
        TradeState::RequestCreated => profile_model.profile.requested_trades_count += 1,
        TradeState::EscrowReleased => profile_model.profile.released_trades_count += 1,
        _ => {}
    }
    profile_model.profile.last_trade = env.block.time.seconds();
    let profile = profile_model.save();

    let res = Response::new()
        .add_attribute("action", "increase_trades_count")
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

fn register_hub(
    deps: DepsMut<KujiraQuery>,
    info: MessageInfo,
) -> Result<Response<KujiraMsg>, ContractError> {
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
pub fn migrate(
    _deps: DepsMut<KujiraQuery>,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    Ok(Response::default()
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("name", CONTRACT_NAME))
}
