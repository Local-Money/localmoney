#![cfg(test)]

use crate::contract::{execute, instantiate, query};
use crate::errors::GovernanceError;
use crate::mock_querier::mock_dependencies;
use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{from_binary, to_binary, Addr, DepsMut, Empty, Response, Uint128};
use cosmwasm_vm::testing::mock_env;
use cw20::Cw20ReceiveMsg;
use localterra_protocol::governance::{
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg, Staker, State,
};
use std::ops::Add;

fn cw20_send(
    deps: DepsMut,
    token: &str,
    sender: String,
    amount: Uint128,
    msg: Cw20HookMsg,
) -> Result<Response<Empty>, GovernanceError> {
    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender,
        amount,
        msg: to_binary(&msg).unwrap(),
    });
    execute(deps, mock_env(), mock_info(token, &[]), msg)
}

#[test]
fn test_stake_and_withdraw() {
    let staker_balance = Uint128::new(1000u128);
    let mut deps = mock_dependencies(&[], None);

    let gov_owner = mock_info("factory", &[]);
    let staker = mock_info("staker", &[]);

    let token_addr = Addr::unchecked("local");

    //Instantiate Gov Contract.
    let gov_init_msg = InstantiateMsg {};

    let gov_env = mock_env();
    let res = instantiate(
        deps.as_mut(),
        gov_env.clone(),
        gov_owner.clone(),
        gov_init_msg,
    );
    assert!(res.is_ok());
    deps.querier.with_token_balances(&[
        (
            &token_addr.to_string(),
            &[(&staker.sender.to_string(), &staker_balance.clone())],
        ),
        (
            &token_addr.to_string(),
            &[(
                &gov_env.contract.address.to_string(),
                &staker_balance.clone(),
            )],
        ),
    ]);

    //Send Tokens from Staker to the Governance contract and assert that it's ok.
    let res = cw20_send(
        deps.as_mut(),
        token_addr.as_str(),
        staker.sender.to_string(),
        staker_balance.clone(),
        Cw20HookMsg::StakeTokens {},
    );
    assert!(res.is_ok());

    //Check that Staker's and Total shares were increased.
    let state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    let staker_info: Staker = from_binary(
        &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Staker {
                address: staker.sender.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_ne!(staker_info.shares, Uint128::zero());
    assert_eq!(staker_info.shares, state.total_shares);

    //Withdraw shares.
    let res = execute(
        deps.as_mut(),
        mock_env(),
        staker.clone(),
        ExecuteMsg::Withdraw {
            shares: staker_info.shares,
        },
    );
    assert!(res.is_ok());

    //Check that Staker's and Total shares were reduced.
    let state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    let staker_info: Staker = from_binary(
        &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Staker {
                address: staker.sender.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(staker_info.shares, Uint128::zero());
    assert_eq!(staker_info.shares, state.total_shares);
}

#[test]
fn test_deposit_rewards() {
    let staker_balance = Uint128::new(1000u128);
    let mut deps = mock_dependencies(&[], None);

    let staker_a = mock_info("staker_a", &[]);
    let staker_b = mock_info("staker_b", &[]);
    let factory = mock_info("factory", &[]);
    let token_addr = Addr::unchecked("local");

    deps.querier.with_token_balances(&[
        (
            &"LOCAL".to_string(),
            &[(&staker_a.sender.to_string(), &staker_balance)],
        ),
        (
            &"LOCAL".to_string(),
            &[(&staker_b.sender.to_string(), &staker_balance)],
        ),
        (
            &"LOCAL".to_string(),
            &[(&factory.sender.to_string(), &staker_balance)],
        ),
    ]);

    //Instantiate Gov Contract.
    let gov_init_msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), mock_env(), factory.clone(), gov_init_msg);

    //Send tokens from Staker A to the Gov contract and assert that it's ok.
    let res = cw20_send(
        deps.as_mut(),
        token_addr.as_str(),
        staker_a.sender.to_string(),
        staker_balance.clone(),
        Cw20HookMsg::StakeTokens {},
    );
    assert!(res.is_ok());

    //Deposit tokens on the Gov contract to simulate RewardsDeposit.
    let res = cw20_send(
        deps.as_mut(),
        token_addr.as_str(),
        factory.sender.to_string(),
        staker_balance.clone(),
        Cw20HookMsg::DepositRewards {},
    );
    assert!(res.is_ok());

    //Sent tokens from Staker B to teh Gov contract and assert that it's ok.
    let res = cw20_send(
        deps.as_mut(),
        token_addr.as_str(),
        staker_b.sender.to_string(),
        staker_balance.clone(),
        Cw20HookMsg::StakeTokens {},
    );
    assert!(res.is_ok());

    //Check that state.total_shares == staker_a.shares + staker_b.shares.
    let state: State =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap();
    let staker_a_info: Staker = from_binary(
        &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Staker {
                address: staker_a.sender.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    let staker_b_info: Staker = from_binary(
        &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Staker {
                address: staker_b.sender.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_ne!(staker_a_info.shares, Uint128::zero());
    assert_eq!(staker_a_info.shares, staker_b_info.shares);
    assert_eq!(
        state.total_shares,
        staker_a_info.shares.add(staker_b_info.shares)
    );
}
