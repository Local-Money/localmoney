#![cfg(test)]

use crate::contract::{execute, instantiate, query};
use cosmwasm_std::{from_binary, to_binary, Uint128};
use cosmwasm_vm::testing::{mock_env, mock_info};
use cw20::Cw20ReceiveMsg;
use localterra_protocol::mock_querier::mock_dependencies;
use localterra_protocol::trading_incentives::{
    Cw20HookMsg, Distribution, ExecuteMsg as TradingIncentivesMsg, ExecuteMsg, InstantiateMsg,
    QueryMsg,
};

#[test]
fn distribution_happy_path() {
    let mut deps = mock_dependencies(&[], None);
    let mut env = mock_env();
    let info = mock_info("factory", &[]);

    let instantiate_msg = InstantiateMsg {};

    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg);
    assert!(res.is_ok());

    let res = execute(
        deps.as_mut(),
        env.clone(),
        mock_info("local", &[]),
        ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: "local".to_string(),
            amount: Uint128::new(1_000_000u128),
            msg: to_binary(&Cw20HookMsg::StartDistribution {}).unwrap(),
        }),
    );
    assert!(res.is_ok());

    let distribution: Distribution =
        from_binary(&query(deps.as_ref(), env.clone(), QueryMsg::Distribution {}).unwrap())
            .unwrap();
    assert_eq!(distribution.current_period, 0);

    let new_block_time = env
        .block
        .time
        .plus_seconds(distribution.period_duration + 1);
    env.block.time = new_block_time;
    let distribution: Distribution =
        from_binary(&query(deps.as_ref(), env.clone(), QueryMsg::Distribution {}).unwrap())
            .unwrap();
    assert_eq!(distribution.current_period, 1);

    let new_block_time = env
        .block
        .time
        .plus_seconds(distribution.period_duration * 50);
    env.block.time = new_block_time;
    let distribution: Distribution =
        from_binary(&query(deps.as_ref(), env.clone(), QueryMsg::Distribution {}).unwrap())
            .unwrap();
    assert_eq!(distribution.current_period, 51);

    //Check that it won't go further than max periods.
    let new_block_time = env
        .block
        .time
        .plus_seconds(distribution.period_duration * 100);
    env.block.time = new_block_time;
    let distribution: Distribution =
        from_binary(&query(deps.as_ref(), env.clone(), QueryMsg::Distribution {}).unwrap())
            .unwrap();
    assert_eq!(distribution.current_period, 51);
}

#[test]
fn test_claim() {
    let mut deps = mock_dependencies(&[], None);
    let mut env = mock_env();
    let info = mock_info("factory", &[]);
    let local_info = mock_info("local", &[]);

    let instantiate_msg = InstantiateMsg {};

    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg);
    assert!(res.is_ok());

    let res = execute(
        deps.as_mut(),
        env.clone(),
        local_info.clone(),
        ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: "local".to_string(),
            amount: Uint128::new(1_000_000u128),
            msg: to_binary(&Cw20HookMsg::StartDistribution {}).unwrap(),
        }),
    );
    assert!(res.is_ok());

    let distribution: Distribution =
        from_binary(&query(deps.as_ref(), env.clone(), QueryMsg::Distribution {}).unwrap())
            .unwrap();
    assert_eq!(distribution.current_period, 0);

    //Register Trade for Maker A, Maker B and check their rewards.
    let maker_a = mock_info("maker_a", &[]);

    //Create Trade Registration message to be sent to the Trading Incentives contract.
    //Register Maker A twice (to increment its shares in relation to Maker B)
    let res = execute(
        deps.as_mut(),
        env.clone(),
        maker_a.clone(),
        TradingIncentivesMsg::RegisterTrade {
            trade: env.contract.address.to_string(),
            maker: maker_a.sender.to_string(),
        },
    );
    assert!(res.is_ok());

    env.block.time = env
        .block
        .time
        .plus_seconds(distribution.period_duration * 2);
    let res = execute(
        deps.as_mut(),
        env.clone(),
        maker_a.clone(),
        TradingIncentivesMsg::Claim { period: 0 },
    );
    assert!(res.is_ok());
}
