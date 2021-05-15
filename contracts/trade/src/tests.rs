use crate::contract::{init, query};
use crate::mock_querier::{mock_dependencies_custom, test_offer, OfferType};
use crate::msg::{ConfigResponse, InitMsg, QueryMsg};
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{
    from_binary, Api, Empty, Env, Extern, HumanAddr, InitResponse, Querier, Storage,
};

fn do_init<S: Storage, A: Api, Q: Querier>(
    mut deps: &mut Extern<S, A, Q>,
    env: Env,
) -> InitResponse<Empty> {
    let offer_contract = HumanAddr::from("offer");
    let init_msg = InitMsg {
        offer_contract,
        offer: 0,
        amount: 1000,
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
}
