use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_std::{
    from_binary, Api, Coin, Empty, Env, Extern, HumanAddr, InitResponse, MessageInfo, Querier,
    StdError, Storage,
};

use crate::contract::{handle, init, query};
use crate::msg::{ConfigResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::TradeState;
use cosmwasm_vm::testing::mock_info;
use StdError::Unauthorized;
//TODO: Will probably get deleted and replaced by the integration test
/*
fn do_init<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    env: Env,
    info: MessageInfo,
) -> InitResponse<Empty> {
    let offer_contract = HumanAddr::from("offer");
    let test_offer = test_offer();
    let init_msg = InitMsg {
        offer_contract,
        offer: 0,
        amount: test_offer.max_amount.to_string().parse::<u64>().unwrap(),
    };
    let res = init(&mut deps, env, info, init_msg).unwrap();

    assert_eq!(res.messages.len(), 0);
    return res;
    //TODO: Remove
    return InitResponse::default()
}

#[test]
fn proper_init() {
    //let mut deps = mock_dependencies_custom(20, &[]);
    let mut deps = mock_dependencies(&[]);
    let test_offer = test_offer();
    let owner = HumanAddr::from("owner");
    let info = mock_info(owner.clone(), &[]);

    let res = do_init(&mut deps, mock_env(), info.clone());
    assert_eq!(res, InitResponse::default());

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse = from_binary(&query(&deps, query_config).unwrap()).unwrap();

    if test_offer.offer_type == OfferType::Buy {
        assert_eq!(conf.sender, owner.clone())
    } else {
        assert_eq!(conf.recipient, owner.clone())
    }

    assert_eq!(conf.state, TradeState::Created)
}

#[test]
fn init_with_funds() {
    let test_offer = test_offer();
    let coins = &[Coin {
        amount: test_offer.max_amount,
        denom: "uusd".to_string(),
    }];
    let mut deps = mock_dependencies_custom(20, coins);
    let owner = HumanAddr::from("owner");
    let info = mock_info(owner.clone(), coins);

    let res = do_init(&mut deps, mock_env(), info.clone());
    assert_eq!(res, InitResponse::default());

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse = from_binary(&query(&deps, query_config).unwrap()).unwrap();
    assert_eq!(conf.state, TradeState::EscrowFunded)
}

#[test]
fn release_funds() {
    let mut test_offer = test_offer();
    //TODO:
    //test_offer.offer_type = OfferType::Sell;

    let coins = &[Coin {
        amount: test_offer.max_amount,
        denom: "uusd".to_string(),
    }];

    let mut deps = mock_dependencies_custom(20, coins);
    let owner = HumanAddr::from("owner");
    let info = mock_info(owner.clone(), coins);

    let res = do_init(&mut deps, mock_env(), info.clone());
    assert_eq!(res, InitResponse::default());

    let release_msg = HandleMsg::Release {};

    let other = HumanAddr::from("other");
    let other_info = mock_info(other.clone(), coins);
    let res = handle(&mut deps, mock_env(), other_info.clone(), release_msg);
    assert_eq!(res.err(), Some(Unauthorized { backtrace: None }));

    let release_msg = HandleMsg::Release {};
    let res = handle(&mut deps, mock_env(), info.clone(), release_msg);
    assert_eq!(res.err(), None);

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse = from_binary(&query(&deps, query_config).unwrap()).unwrap();
    assert_eq!(conf.state, TradeState::Closed);
}

#[test]
fn trade_expiration() {
    let mut test_offer = test_offer();
    //TODO:
    //test_offer.offer_type = OfferType::Sell;

    let coins = &[Coin {
        amount: test_offer.max_amount,
        denom: "uusd".to_string(),
    }];

    let mut deps = mock_dependencies_custom(20, coins);
    let owner = HumanAddr::from("owner");
    let mut info = mock_info(owner.clone(), &[]);

    let res = do_init(&mut deps, mock_env(), info.clone());
    assert_eq!(res, InitResponse::default());

    let refund_msg = HandleMsg::Refund {};
    let res = handle(&mut deps, mock_env(), info.clone(), refund_msg);
    match res {
        Err(StdError::GenericErr {
            msg,
            backtrace: _backtrace,
        }) => {
            assert_eq!(msg, "Can't release an unexpired Trade.");
        }
        _ => panic!("Unexpected error"),
    }

    //TODO: get expiration from the contract
    let mut env = mock_env();
    env.block.height += 101;

    let release_msg = HandleMsg::Release {};
    let res = handle(&mut deps, env.clone(), info.clone(), release_msg);
    match res {
        Err(StdError::GenericErr {
            msg,
            backtrace: _backtrace,
        }) => {
            assert_eq!(msg, "This trade has expired");
        }
        _ => panic!("Unexpected error"),
    }

    let refund_msg = HandleMsg::Refund {};
    let res = handle(&mut deps, env.clone(), info.clone(), refund_msg);
    assert_eq!(res.err(), None);
}

*/
