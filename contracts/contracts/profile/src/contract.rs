use crate::state::PROFILE;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::HubAlreadyRegistered;
use localterra_protocol::guards::assert_ownership;
use localterra_protocol::hub_utils::{get_hub_config, register_hub_internal};
use localterra_protocol::profile::{ExecuteMsg, InstantiateMsg, MigrateMsg, Profile, QueryMsg};
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
        ExecuteMsg::IncreaseTradeCount {
            profile_address,
            final_trade_state,
        } => increase_trade_count(deps, info, profile_address, final_trade_state),
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
    }
}

//
pub fn increase_trade_count(
    deps: DepsMut,
    info: MessageInfo,
    profile_address: Addr,
    _final_trade_state: TradeState,
) -> Result<Response, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());

    // Only the trade contract should be able to call increase_trade_count
    assert_ownership(info.sender, hub_config.trade_addr).unwrap();

    let profile = PROFILE
        .may_load(deps.storage, profile_address.to_string())
        .unwrap();

    let mut profile = profile.unwrap_or(Profile::new(profile_address.clone()));
    profile.trade_count += 1;

    PROFILE
        .save(deps.storage, profile_address.to_string(), &profile)
        .unwrap();

    let res = Response::new()
        .add_attribute("action", "increase_trade_count")
        .add_attribute("trade_count", profile.trade_count.to_string());
    Ok(res)
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Profile { address } => to_binary(&query_profile(deps, address)?),
        QueryMsg::Profiles {} => to_binary(&query_profiles(deps)?),
    }
}

fn query_profile(deps: Deps, profile_address: Addr) -> StdResult<Profile> {
    let profile = PROFILE
        .may_load(deps.storage, profile_address.to_string())
        .unwrap();

    let profile = profile.unwrap_or(Profile::new(profile_address));

    Ok(profile)
}

fn query_profiles(_deps: Deps) -> StdResult<Response> {
    // TODO list all profiles
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default()
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("name", CONTRACT_NAME))
}
