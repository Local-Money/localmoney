#[cfg(test)]
use crate::contract::{execute, instantiate, query};
use crate::mock_querier::mock_dependencies;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Uint128};
use localterra_protocol::fee_collector::ExecuteMsg::UpdateConfig;
use localterra_protocol::fee_collector::{Config, ExecuteMsg, InstantiateMsg, QueryMsg};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[], None);
    let info = mock_info("factory", &coins(1000, "uusd"));

    let msg = InstantiateMsg {
        ust_conversion_threshold: Uint128::new(100000),
    };

    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let config: Config = from_binary(&res).unwrap();

    assert_eq!(Uint128::new(100000), config.ust_conversion_threshold);
    assert_eq!("factory".to_string(), config.factory_addr);
    //assert_eq!("gov-addr".to_string(), config.gov_addr);
}

#[test]
fn execute_update_config() {
    let mut deps = mock_dependencies(&[], None);
    let info = mock_info("factory", &coins(1000, "uusd"));

    let msg = InstantiateMsg {
        ust_conversion_threshold: Uint128::new(100000),
    };

    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let info = mock_info("factory", &coins(1000, "uusd"));

    let msg = UpdateConfig {
        ust_conversion_threshold: Uint128::new(500000),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let config: Config = from_binary(&res).unwrap();

    assert_eq!(Uint128::new(500000), config.ust_conversion_threshold);
    assert_eq!("factory".to_string(), config.factory_addr);
}

#[test]
fn execute_distribute_fee() {
    let env = mock_env();
    let mut deps = mock_dependencies(&coins(1000, "uusd"), None);
    let info = mock_info("factory", &coins(1000, "uusd"));

    // Instantiate contract with 100000 as conversion threshold
    let msg = InstantiateMsg {
        ust_conversion_threshold: Uint128::new(100000),
    };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Try to distribute fee with less then the conversion threshold and get an error
    let msg = ExecuteMsg::Distribute {};
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg);
    assert!(res.is_err());

    // Distribute fee successfully when the contract have at least the conversion threshold value
    deps.querier
        .update_balance(env.contract.address.clone(), coins(100000, "uusd"));
    let msg = ExecuteMsg::Distribute {};
    let res = execute(deps.as_mut(), env, info, msg);
    assert!(res.is_ok());
}
