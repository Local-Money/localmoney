use crate::error::ContractError;
use crate::state::{config_read, config_storage};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, ContractResult, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply,
    ReplyOn, Response, StdError, StdResult, SubMsg, SubMsgExecutionResponse, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use terraswap::asset::{Asset, AssetInfo};
use terraswap::pair::ExecuteMsg::Swap;

use localterra_protocol::fee_collector::{Config, ExecuteMsg, InstantiateMsg, QueryMsg};
use localterra_protocol::governance::Cw20HookMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let local_ust_pool_addr = deps.api.addr_validate(&msg.local_ust_pool_addr).unwrap();
    let gov_addr = deps.api.addr_validate(&msg.gov_addr).unwrap();
    let ust_conversion_threshold = Uint128::new(msg.ust_conversion_threshold);

    let cfg = Config {
        ust_conversion_threshold,
        local_ust_pool_addr,
        gov_addr,
    };
    config_storage(deps.storage).save(&cfg).unwrap();

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Distribute {} => distribute_fee(deps, env),
        ExecuteMsg::UpdateConfig {
            ust_conversion_threshold,
            local_ust_pool_addr,
        } => update_config(deps, ust_conversion_threshold, local_ust_pool_addr),
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        0 => send_local_token_to_gov(deps, env, msg.result),
        _ => Err(ContractError::Unauthorized {}),
    }
}

fn distribute_fee(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let cfg = config_read(deps.storage).load().unwrap();

    let ust_balance = deps
        .querier
        .query_balance(env.contract.address, "uusd".to_string())
        .unwrap_or(Coin {
            denom: "uusd".to_string(),
            amount: Uint128::zero(),
        })
        .amount;

    if ust_balance < cfg.ust_conversion_threshold {
        return Err(ContractError::Std(StdError::generic_err(
            "UST balance is below the conversion threshold.",
        )));
    }

    let swap_msg = Swap {
        offer_asset: Asset {
            info: AssetInfo::NativeToken {
                denom: "uusd".to_string(),
            },
            amount: ust_balance.clone(),
        },
        belief_price: None,
        max_spread: None,
        to: None,
    };

    let msg = SubMsg {
        id: 0,
        msg: CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: cfg.local_ust_pool_addr.to_string(),
            msg: to_binary(&swap_msg).unwrap(),
            funds: vec![Coin {
                denom: "uusd".to_string(),
                amount: ust_balance,
            }],
        }),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };
    Ok(Response::new().add_submessage(msg))
}

fn send_local_token_to_gov(
    deps: DepsMut,
    _env: Env,
    result: ContractResult<SubMsgExecutionResponse>,
) -> Result<Response, ContractError> {
    if result.is_err() {
        return Err(ContractError::Unauthorized {});
    }

    let cfg = config_read(deps.storage).load().unwrap();

    let total_local_token = result
        .unwrap()
        .events
        .into_iter()
        .find(|e| e.ty == "from_contract")
        .and_then(|ev| {
            ev.attributes
                .into_iter()
                .find(|attr| attr.key == "return_amount")
        })
        .map(|attr| attr.value.parse::<u128>().unwrap())
        .unwrap();

    let deposit_rewards_msg = Cw20HookMsg::DepositRewards {};

    let cw20msg = Cw20ExecuteMsg::Send {
        contract: cfg.gov_addr.to_string(),
        amount: Uint128::new(total_local_token),
        msg: to_binary(&deposit_rewards_msg).unwrap(),
    };

    let msg = SubMsg::new(WasmMsg::Execute {
        contract_addr: cfg.gov_addr.to_string(),
        msg: to_binary(&cw20msg).unwrap(),
        funds: vec![],
    });

    let res = Response::default().add_submessage(msg);

    Ok(res)
}

fn update_config(
    deps: DepsMut,
    conversion_threshold: u128,
    pool_addr: String,
) -> Result<Response, ContractError> {
    let config = config_read(deps.storage).load().unwrap();

    let local_ust_pool_addr = deps.api.addr_validate(&pool_addr).unwrap();
    let ust_conversion_threshold = Uint128::new(conversion_threshold);
    let gov_addr = config.gov_addr;

    let cfg = Config {
        ust_conversion_threshold,
        local_ust_pool_addr,
        gov_addr,
    };
    config_storage(deps.storage).save(&cfg).unwrap();

    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let cfg = config_read(deps.storage).load().unwrap();
    Ok(cfg)
}
