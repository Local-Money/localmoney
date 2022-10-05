#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::HubAlreadyRegistered;
use localterra_protocol::guards::assert_ownership;
use localterra_protocol::hub_utils::{get_hub_config, register_hub_internal};
use localterra_protocol::profile::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, Profile, ProfileModel, QueryMsg,
};
use localterra_protocol::trade::TradeState;

// version info for migration info
pub const CONTRACT_NAME: &str = "localmoney.io:profile";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let res = Response::new().add_attribute("action", "instantiate_profile");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Create { profile_address } => create_profile(deps, info, profile_address),
        ExecuteMsg::UpdateProfile {
            profile_address,
            contact,
            encrypt_pk,
        } => update_profile(deps, info, profile_address, contact, encrypt_pk),
        ExecuteMsg::IncreaseTradeCount {
            profile_address,
            final_trade_state,
        } => increase_trade_count(deps, info, profile_address, final_trade_state),
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
    }
}

fn create_profile(
    deps: DepsMut,
    info: MessageInfo,
    profile_address: Addr,
) -> Result<Response, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());
    // Only the trade contract should be able to call this method
    assert_ownership(info.sender, hub_config.trade_addr).unwrap();

    // Only creates a new profile if it's not already created
    if !ProfileModel::has(deps.storage, profile_address.to_string()) {
        ProfileModel::store(deps.storage, &Profile::new(profile_address.clone()));
    }

    let res = Response::new()
        .add_attribute("action", "create_profile")
        .add_attribute("profile_address", profile_address.to_string());
    Ok(res)
}

fn update_profile(
    deps: DepsMut,
    _info: MessageInfo,
    profile_address: Addr,
    contact: String,
    encrypt_pk: String,
) -> Result<Response, ContractError> {
    // TODO Set the ownership to this method
    // let hub_config = get_hub_config(deps.as_ref());
    // Only the trade contract should be able to call this method
    // assert_ownership(info.sender, hub_config.trade_addr).unwrap();

    let mut profile_model = ProfileModel::load(deps.storage, profile_address.clone());
    profile_model.profile.contact = Some(contact.clone());
    profile_model.profile.encrypt_pk = Some(encrypt_pk.clone());
    profile_model.save();

    let res = Response::new()
        .add_attribute("action", "update_profile")
        .add_attribute("profile_address", profile_address.to_string())
        .add_attribute("contact", contact.to_string())
        .add_attribute("encrypt_pk", encrypt_pk.to_string());
    Ok(res)
}

pub fn increase_trade_count(
    deps: DepsMut,
    info: MessageInfo,
    profile_address: Addr,
    final_trade_state: TradeState,
) -> Result<Response, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());

    // Only the trade contract should be able to call increase_trade_count
    assert_ownership(info.sender, hub_config.trade_addr).unwrap();

    let mut profile_model = ProfileModel::load(deps.storage, profile_address.clone());
    profile_model.profile.trade_count += 1;
    let updated_profile = profile_model.save();

    let res = Response::new()
        .add_attribute("action", "increase_trade_count")
        .add_attribute("final_trade_state", final_trade_state.to_string())
        .add_attribute("trade_count", updated_profile.trade_count.to_string());
    Ok(res)
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Profile { address } => to_binary(&query_profile(deps, address)?),
    }
}

fn query_profile(deps: Deps, profile_address: Addr) -> StdResult<Profile> {
    let profile = ProfileModel::query(deps.storage, profile_address);
    Ok(profile)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default()
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("name", CONTRACT_NAME))
}
