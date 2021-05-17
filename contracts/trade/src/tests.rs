use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{
    from_binary, Api, Coin, Empty, Env, Extern, HumanAddr, InitResponse, Querier, Storage,
};

use crate::contract::{init, query};
use crate::mock_querier::{mock_dependencies_custom, test_offer, OfferType};
use crate::msg::{ConfigResponse, InitMsg, QueryMsg};
use crate::state::TradeState;

fn do_init<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    env: Env,
) -> InitResponse<Empty> {
    let offer_contract = HumanAddr::from("offer");
    let test_offer = test_offer();
    let init_msg = InitMsg {
        offer_contract,
        offer: 0,
        amount: test_offer.max_amount.to_string().parse::<u64>().unwrap(),
    };
    let res = init(&mut deps, env, init_msg).unwrap();

    assert_eq!(res.messages.len(), 0);
    return res;
}

#[test]
fn proper_init() {
    let mut deps = mock_dependencies_custom(20, &[]);
    let test_offer = test_offer();
    let owner = HumanAddr::from("owner");
    let env = mock_env(owner.clone(), &[]);

    let res = do_init(&mut deps, env.clone());
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
    let env = mock_env(owner.clone(), coins);
    let res = do_init(&mut deps, env.clone());
    assert_eq!(res, InitResponse::default());

    let query_config = QueryMsg::Config {};
    let conf: ConfigResponse = from_binary(&query(&deps, query_config).unwrap()).unwrap();
    assert_eq!(conf.state, TradeState::EscrowFunded)
}
