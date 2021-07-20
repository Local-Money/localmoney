use crate::errors::TradeError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, OfferMsg, QueryMsg};
use crate::state::{config, config_read, State, TradeState};
use crate::taxation::deduct_tax;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Attribute, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps,
    DepsMut, Env, MessageInfo, QueryRequest, Response, StdResult, SubMsg, Uint128, WasmMsg,
    WasmQuery,
};
use cw20::Cw20ReceiveMsg;
use offer::state::{Offer, OfferType};
use terraswap::asset::{Asset, AssetInfo, AssetInfo::Token as TokenInfo, PairInfo};
use terraswap::pair::ExecuteMsg::Swap;
use terraswap::pair::SimulationResponse;
use terraswap::querier::{query_pair_info, simulate};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, TradeError> {
    //Load Offer
    let offer_id = msg.offer_id;
    let load_offer_result: StdResult<Offer> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: msg.offer_contract.to_string(),
            msg: to_binary(&OfferMsg::LoadOffer { id: offer_id }).unwrap(),
        }));
    if load_offer_result.is_err() {
        return Err(TradeError::OfferNotFound { offer_id });
    }
    let offer = load_offer_result.unwrap();

    //TODO: it's probably a good idea to store this kind of configuration in a Gov contract.
    let expire_height = env.block.height + 600; //Roughly 1h.

    //Check that ust_amount is inside Offer limits
    let amount = Uint128::from(msg.ust_amount);
    if amount > offer.max_amount || amount < offer.min_amount {
        return Err(TradeError::AmountError {
            amount,
            min_amount: offer.min_amount,
            max_amount: offer.max_amount,
        });
    }

    //Instantiate recipient and sender addresses according to Offer type (buy, sell)
    let recipient: Addr;
    let sender: Addr;
    let _taker_is_buying: bool;

    if offer.offer_type == OfferType::Buy {
        _taker_is_buying = false;
        recipient = offer.owner;
        sender = info.sender.clone();
    } else {
        _taker_is_buying = true;
        recipient = info.sender.clone();
        sender = offer.owner;
    }

    //Instantiate Trade state
    let mut state = State {
        recipient,
        sender,
        offer_id,
        state: TradeState::Created,
        expire_height,
        ust_amount: Uint128::from(msg.ust_amount),
        final_asset: msg.final_asset,
        terraswap_factory: msg.terraswap_factory,
    };

    //Set state to EscrowFunded if enough UST was sent in the message.
    if !info.funds.is_empty() {
        //TODO: Check for Luna or other Terra native tokens.
        let ust_amount = get_ust_amount(info.clone());
        if ust_amount >= Uint128::from(msg.ust_amount) {
            state.state = TradeState::EscrowFunded
        }
    }

    //Save state.
    let save_state_result = config(deps.storage).save(&state);
    if save_state_result.is_err() {
        return Err(TradeError::InstantiationError {
            message: "Couldn't save state.".to_string(),
        });
    }

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, TradeError> {
    let cfg = config(deps.storage);
    let mut state = cfg.load().unwrap();
    match msg {
        ExecuteMsg::FundEscrow {} => try_fund_escrow(deps, env, info, state),
        ExecuteMsg::Refund {} => try_refund(deps, env, state),
        ExecuteMsg::Release {} => try_release(deps, env, info, state),
        ExecuteMsg::Receive(msg) => try_receive_cw20(deps, &mut state, env, msg),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(state)
}

fn try_fund_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut state: State,
) -> Result<Response, TradeError> {
    let ust_amount = if !info.funds.is_empty() {
        get_ust_amount(info.clone())
    } else {
        let ust_balance = deps
            .querier
            .query_balance(env.contract.address, "uusd".to_string());
        ust_balance
            .unwrap_or(Coin {
                denom: "uusd".to_string(),
                amount: Uint128::zero(),
            })
            .amount
    };
    if ust_amount >= state.ust_amount {
        state.state = TradeState::EscrowFunded;
    } else {
        return Err(TradeError::ExecutionError {
            message: "UST amount is less than required to fund the escrow.".to_string(),
        });
    }
    let save_result = config(deps.storage).save(&state);
    if save_result.is_err() {
        return Err(TradeError::ExecutionError {
            message: "Failed to save state.".to_string(),
        });
    }
    Ok(Response::default())
}

fn try_release(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    state: State,
) -> Result<Response, TradeError> {
    if info.sender != state.sender {
        return Err(TradeError::Unauthorized {
            owner: state.sender,
            caller: info.sender,
        });
    }

    // throws error if state is expired
    if env.block.height > state.expire_height {
        return Err(TradeError::Expired {
            expire_height: state.expire_height,
            current_height: env.block.height,
        });
    }

    let balance_result = deps.querier.query_all_balances(&env.contract.address);
    if balance_result.is_err() {
        return Err(TradeError::ReleaseError {
            message: "Contract has no funds.".to_string(),
        });
    }

    //Update trade State to TradeState::Closed.
    let balance = balance_result.unwrap();
    let mut cfg = config(deps.storage);
    let mut state = cfg.load().unwrap();
    state.state = TradeState::Closed;
    let save_result = cfg.save(&state);
    if save_result.is_err() {
        return Err(TradeError::ExecutionError {
            message: "Failed to save state.".to_string(),
        });
    }

    if None == state.final_asset || "uusd" == state.final_asset.clone().unwrap() {
        send_tokens(deps, state.recipient.clone(), balance, "approve")
    } else {
        match deps
            .api
            .addr_validate(state.final_asset.clone().unwrap().as_str())
        {
            Ok(cw20addr) => convert_to_cw20(deps, &mut state, env, cw20addr),
            Err(_) => Err(TradeError::ExecutionError {
                message: "Invalid cw20 addr or unsupported asset.".to_string(),
            }),
        }
    }
}

fn try_refund(deps: DepsMut, env: Env, state: State) -> Result<Response, TradeError> {
    // anyone can try to refund, as long as the contract is expired
    if state.expire_height > env.block.height {
        return Err(TradeError::RefundError {
            message: "Only expired trades can be refunded.".to_string(),
        });
    }

    let balance_result = deps.querier.query_all_balances(&env.contract.address);
    return if balance_result.is_ok() {
        let balance = balance_result.unwrap();
        send_tokens(deps, state.sender, balance, "refund")
    } else {
        Err(TradeError::RefundError {
            message: "Contract has no funds.".to_string(),
        })
    };
}

fn try_receive_cw20(
    deps: DepsMut,
    state: &mut State,
    env: Env,
    msg: Cw20ReceiveMsg,
) -> Result<Response, TradeError> {
    let final_asset = state
        .final_asset
        .clone()
        .unwrap_or("uusd".to_string())
        .clone();
    match final_asset.as_str() {
        //TODO: Use array of supported stablecoins instead.
        "uusd" => convert_to_ust(deps, state, env, &msg),
        _asset_address => Err(TradeError::ExecutionError {
            message: "Final asset must be UST (uusd) when sending CW20 Tokens.".to_string(),
        }),
    }
}

fn get_ust_amount(info: MessageInfo) -> Uint128 {
    let mut ust_amount = Uint128::zero();
    let ust_index: &Option<usize> = &info.funds.iter().position(|coin| coin.denom.eq("uusd"));
    if Into::<usize>::into(ust_index.unwrap()) >= usize::MIN {
        let ust_coin: &Coin = &info.funds[ust_index.unwrap()];
        ust_amount = ust_coin.amount;
    }
    return ust_amount;
}

// this is a helper to move the tokens, so the business logic is easy to read
fn send_tokens(
    deps: DepsMut,
    to_address: Addr,
    amount: Vec<Coin>,
    action: &str,
) -> Result<Response, TradeError> {
    let attributes = vec![attr("action", action), attr("to", to_address.clone())];
    let amount = [deduct_tax(&deps.querier, amount[0].clone()).unwrap()].to_vec();

    let r = Response {
        messages: vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: to_address.to_string(),
            amount,
        }))],
        data: None,
        attributes,
        events: vec![],
    };
    Ok(r)
}

fn convert_to_ust(
    deps: DepsMut,
    state: &mut State,
    _env: Env,
    cw20_msg: &Cw20ReceiveMsg,
) -> Result<Response, TradeError> {
    let token_address = deps.api.addr_validate(&cw20_msg.sender).unwrap();

    //Check if Pair exists
    let assets_infos = &[
        TokenInfo {
            contract_addr: token_address.clone(),
        },
        AssetInfo::NativeToken {
            denom: "uusd".to_string(),
        },
    ];
    let terraswap_factory_addr = &state.terraswap_factory.clone().unwrap();
    let pair_info_res =
        query_pair_info(&deps.querier, terraswap_factory_addr.clone(), assets_infos);
    let pair_info: PairInfo;
    match pair_info_res {
        Ok(info) => {
            pair_info = info;
        }
        Err(_) => {
            return Err(TradeError::ExecutionError {
                message: "Failed to query TerraSwap pair info.".to_string(),
            });
        }
    };

    //Simulate Swap for UST, if UST amount is >= than the ust_amount, proceed with the swap.
    let asset = Asset {
        info: AssetInfo::Token {
            contract_addr: token_address.clone(),
        },
        amount: cw20_msg.amount.clone(),
    };
    let swap_simulation: SimulationResponse;
    let swap_simulation_res = simulate(&deps.querier, pair_info.contract_addr.clone(), &asset);
    match swap_simulation_res {
        Ok(res) => {
            swap_simulation = res;
            if swap_simulation.return_amount < state.ust_amount {
                return Err(TradeError::SwapError {
                    required_amount: state.ust_amount,
                    returned_amount: swap_simulation.return_amount.clone(),
                });
            }
        }
        Err(_) => {
            return Err(TradeError::ExecutionError {
                message: "Failed to simulate swap.".to_string(),
            })
        }
    }

    if swap_simulation.return_amount.clone() >= state.ust_amount {
        //Send CW20 to Pair contract to convert it to UST
        let swap_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: pair_info.contract_addr.clone().to_string(),
            msg: to_binary(&Swap {
                offer_asset: Asset {
                    info: asset.info.clone(),
                    amount: cw20_msg.amount,
                },
                belief_price: None,
                max_spread: Some(Decimal::percent(1u64)),
                to: None,
            })
            .unwrap(),
            funds: vec![],
        });

        let swap_sub_msg = SubMsg::new(swap_msg);
        Ok(Response {
            messages: vec![swap_sub_msg],
            attributes: vec![],
            events: vec![],
            data: None,
        })
    } else {
        Err(TradeError::SwapError {
            required_amount: state.ust_amount.clone(),
            returned_amount: swap_simulation.return_amount.clone(),
        })
    }
}

fn convert_to_cw20(
    deps: DepsMut,
    state: &mut State,
    env: Env,
    token_address: Addr,
) -> Result<Response, TradeError> {
    //Check if Pair exists
    let assets_infos = &[
        TokenInfo {
            contract_addr: token_address.clone(),
        },
        AssetInfo::NativeToken {
            denom: "uusd".to_string(),
        },
    ];
    let terraswap_factory_addr = &state.terraswap_factory.clone().unwrap();
    let pair_info_res =
        query_pair_info(&deps.querier, terraswap_factory_addr.clone(), assets_infos);
    let pair_info: PairInfo;
    match pair_info_res {
        Ok(info) => {
            pair_info = info;
        }
        Err(_) => {
            return Err(TradeError::ExecutionError {
                message: "Failed to query TerraSwap pair info.".to_string(),
            });
        }
    };

    //Simulate Swap for cw20.
    //Query UST Balance
    let ust_balance = deps
        .querier
        .query_balance(env.contract.address.clone(), "uusd".to_string());
    if ust_balance.is_err() {
        return Err(TradeError::ReleaseError {
            message: "Not enough UST to convert".to_string(),
        });
    }
    let ust_balance = ust_balance.unwrap();
    //Swap Simulation
    let asset = Asset {
        info: AssetInfo::NativeToken {
            denom: "uusd".to_string(),
        },
        amount: state.ust_amount,
    };
    let swap_simulation: SimulationResponse;
    let swap_simulation_res = simulate(&deps.querier, pair_info.contract_addr.clone(), &asset);
    match swap_simulation_res {
        Ok(res) => {
            swap_simulation = res;
            if swap_simulation.return_amount < ust_balance.amount {
                return Err(TradeError::SwapError {
                    required_amount: state.ust_amount,
                    returned_amount: swap_simulation.return_amount.clone(),
                });
            }
        }
        Err(_) => {
            return Err(TradeError::ExecutionError {
                message: "Failed to simulate swap.".to_string(),
            })
        }
    }

    let amount = [deduct_tax(&deps.querier, ust_balance.clone()).unwrap()].to_vec();
    let release_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&ExecuteMsg::Release).unwrap(),
        funds: vec![],
    });
    let release_sub_msg = SubMsg::new(release_msg);
    let _release_sub_msg_id = &release_sub_msg.id.clone();

    let swap_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: pair_info.contract_addr.clone().to_string(),
        msg: to_binary(&Swap {
            offer_asset: asset.clone(),
            belief_price: None,
            max_spread: Some(Decimal::percent(1u64)),
            to: Some(state.recipient.to_string()),
        })
        .unwrap(),
        funds: amount,
    });

    let swap_sub_msg = SubMsg::new(swap_msg);
    let swap_msg_response = Response {
        messages: vec![swap_sub_msg],
        data: None,
        attributes: vec![],
        events: vec![],
    };
    Ok(swap_msg_response)
}

pub fn attr<K: ToString, V: ToString>(key: K, value: V) -> Attribute {
    Attribute {
        key: key.to_string(),
        value: value.to_string(),
    }
}
