#![cfg(test)]
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, TradeState};
use crate::taxation::compute_tax;
use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{coin, to_binary, Addr, BlockInfo, Coin, Empty, Uint128};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg, MinterResponse};
use cw_multi_test::{App, Contract, ContractWrapper, SimpleBank};
use offer::msg::OfferMsg;
use offer::state::{Offer, OfferType};
use serde::de::DeserializeOwned;
use terraswap::asset::{AssetInfo, AssetInfo::Token as AssetInfoToken};

fn mock_app() -> App {
    let env = mock_env();
    let api = Box::new(MockApi::default());
    let bank = SimpleBank {};

    App::new(api, env.block, bank, || Box::new(MockStorage::new()))
}

pub fn offer_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        offer::contract::execute,
        offer::contract::instantiate,
        offer::contract::query,
    );
    Box::new(contract)
}

pub fn trade_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

pub fn terraswap_factory_mock() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::terraswap_factory_mock::execute,
        crate::terraswap_factory_mock::instantiate,
        crate::terraswap_factory_mock::query,
    );
    Box::new(contract)
}

pub fn terraswap_pair_mock() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::terraswap_pair_mock::execute,
        crate::terraswap_pair_mock::instantiate,
        crate::terraswap_pair_mock::query,
    );
    Box::new(contract)
}

struct Vars {
    router: App,
    trade_code_id: u64,
    trade_owner: Addr,
    offer_owner: Addr,
    offer_contract_addr: Addr,
    terraswap_factory_addr: Option<Addr>,
    terraswap_pair_addr: Option<Addr>,
    ust_denom: &'static str,
}

impl Vars {
    pub fn init_terraswap_mock(&mut self, token_contract_addr: Addr) {
        let terraswap_factory_mock_id = self.router.store_code(terraswap_factory_mock());
        let terraswap_pair_mock_id = self.router.store_code(terraswap_pair_mock());

        //Instantiate TerraswapPairMock
        self.terraswap_pair_addr = Some(
            self.router
                .instantiate_contract(
                    terraswap_pair_mock_id,
                    self.trade_owner.clone(),
                    &crate::terraswap_pair_mock::InstantiateMsg {
                        pair: [
                            AssetInfoToken {
                                contract_addr: token_contract_addr.clone(),
                            },
                            AssetInfo::NativeToken {
                                denom: "uusd".to_string(),
                            },
                        ],
                    },
                    &[],
                    "TERRASWAP_PAIR",
                )
                .unwrap(),
        );

        //Instantiate TerraswapFactoryMock
        self.terraswap_factory_addr = Some(
            self.router
                .instantiate_contract(
                    terraswap_factory_mock_id,
                    self.trade_owner.clone(),
                    &crate::terraswap_factory_mock::InstantiateMsg {
                        pair_address: Addr::from(
                            self.terraswap_pair_addr.as_ref().unwrap().clone(),
                        ),
                    },
                    &[],
                    "TERRASWAP_FACTORY",
                )
                .unwrap(),
        );
    }
}

fn init(offer_type: OfferType) -> Vars {
    let mut router = mock_app();
    //Instantiate Contracts Code
    let offer_code_id = router.store_code(offer_contract());
    let trade_code_id = router.store_code(trade_contract());

    let trade_owner = Addr::unchecked("trade_owner");
    let offer_owner = Addr::unchecked("offer_owner");
    let ust_denom = "uusd";

    //Instantiate Offer contract
    let offer_contract_addr = router
        .instantiate_contract(
            offer_code_id,
            offer_owner.clone(),
            &offer::msg::InstantiateMsg {},
            &[],
            "OFFER",
        )
        .unwrap();

    let create_offer_result = router.execute_contract(
        offer_owner.clone(),
        offer_contract_addr.clone(),
        &offer::msg::ExecuteMsg::Create {
            offer: OfferMsg {
                offer_type,
                fiat_currency: offer::currencies::FiatCurrency::COP,
                min_amount: 1_000,
                max_amount: 500_000,
            },
        },
        &[],
    );
    assert!(create_offer_result.is_ok());

    return Vars {
        router,
        trade_code_id,
        trade_owner,
        offer_owner,
        offer_contract_addr,
        terraswap_factory_addr: None,
        terraswap_pair_addr: None,
        ust_denom,
    };
}

fn create_trade(vars: &mut Vars, trade_amount: Uint128) -> Result<Addr, String> {
    return create_trade_with_funds(vars, trade_amount, true, None, vars.ust_denom);
}

fn create_trade_with_funds(
    vars: &mut Vars,
    trade_amount: Uint128,
    send_funds: bool,
    terraswap_factory_addr: Option<Addr>,
    final_asset: &str,
) -> Result<Addr, String> {
    let router = &mut vars.router;

    let instantiate_trade_msg = InstantiateMsg {
        offer_contract: vars.offer_contract_addr.clone(),
        offer_id: 1,
        ust_amount: trade_amount.clone(),
        final_asset: Some(final_asset.to_string()),
        terraswap_factory: terraswap_factory_addr.clone(),
    };

    return if send_funds {
        router.instantiate_contract(
            vars.trade_code_id,
            vars.trade_owner.clone(),
            &instantiate_trade_msg,
            &[Coin {
                denom: vars.ust_denom.to_string(),
                amount: trade_amount,
            }],
            "TRADE",
        )
    } else {
        router.instantiate_contract(
            vars.trade_code_id,
            vars.trade_owner.clone(),
            &instantiate_trade_msg,
            &[],
            "TRADE",
        )
    };
}

fn query_offer_contract<T: DeserializeOwned>(
    vars: &mut Vars,
    query_msg: &offer::msg::QueryMsg,
) -> T {
    return vars
        .router
        .wrap()
        .query_wasm_smart(vars.offer_contract_addr.clone(), query_msg)
        .unwrap();
}

fn query_trade_contract<T: DeserializeOwned>(
    vars: &mut Vars,
    trade_contract_addr: Addr,
    query_msg: &QueryMsg,
) -> T {
    return vars
        .router
        .wrap()
        .query_wasm_smart(trade_contract_addr.clone(), query_msg)
        .unwrap();
}

fn query_ust_balance(vars: &mut Vars, addr: Addr) -> Uint128 {
    return vars
        .router
        .wrap()
        .query_balance(addr.clone(), vars.ust_denom.clone())
        .unwrap()
        .amount;
}

#[test]
fn trade_happy_path() {
    let vars = &mut init(OfferType::Buy);

    let initial_trade_owner_balance = Uint128::new(2_000_000);

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);

    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    //Create Trade
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());
    let trade_contract_addr = create_trade_result.unwrap();

    let trade_amount_coin = Coin {
        denom: vars.ust_denom.to_string(),
        amount: trade_amount,
    };
    //Query Trade contract balance (escrow)
    let trade_contract_balance = vars
        .router
        .wrap()
        .query_balance(trade_contract_addr.clone(), vars.ust_denom.clone())
        .unwrap();
    assert_eq!(trade_contract_balance, trade_amount_coin);

    let terra_tax = &compute_tax(&vars.router.wrap(), &trade_amount_coin).unwrap();

    //Query Trade state and Verify if it's "funded".
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::EscrowFunded);

    let release_response = vars.router.execute_contract(
        vars.trade_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Release {},
        &[],
    );
    assert!(release_response.is_ok());

    //Query Trade state
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});

    let trade_contract_balance = query_ust_balance(vars, trade_contract_addr.clone());
    let trade_owner_balance = query_ust_balance(vars, vars.trade_owner.clone());
    let offer_owner_balance = query_ust_balance(vars, vars.offer_owner.clone());

    assert_eq!(trade_state.state, TradeState::Closed);
    assert_eq!(&trade_contract_balance, terra_tax);
    assert_eq!(
        trade_owner_balance,
        initial_trade_owner_balance
            .checked_sub(trade_amount)
            .unwrap()
    );
    assert_eq!(
        offer_owner_balance,
        trade_amount.checked_sub(terra_tax.clone()).unwrap()
    );
}

#[test]
fn test_trade_amount() {
    let vars = &mut init(OfferType::Buy);

    let initial_trade_owner_balance = Uint128::new(2_000_000);
    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });

    //Create Trade with amount equal to the offer's min amount
    let mut trade_amount = offer_state.min_amount;
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());

    //Create Trade with amount equal to the offer's max amount
    trade_amount = offer_state.max_amount;
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());

    //Create Trade with amount smaller than the offer's min amount
    trade_amount = Uint128::from(offer_state.min_amount.u128() - 1u128);
    let create_trade_result = create_trade(vars, trade_amount);
    assert_eq!(create_trade_result.is_err(), true);

    //Create Trade with amount bigger than the offer's max amount
    let trade_amount = Uint128::from(offer_state.max_amount.u128() + 1u128);
    let create_trade_result = create_trade(vars, trade_amount);
    assert_eq!(create_trade_result.clone().is_err(), true);
}

#[test]
fn test_trade_expiration() {
    let router = mock_app();
    let from = &router.block_info();
    let vars = &mut init(OfferType::Buy);

    let initial_trade_owner_balance = Uint128::new(2_000_000);

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);

    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    //Create Trade
    let ust_trade_amount = coin(trade_amount.clone().u128(), vars.ust_denom);
    let terra_tax = compute_tax(&vars.router.wrap(), &ust_trade_amount).unwrap();
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());
    let trade_contract_addr = create_trade_result.unwrap();
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});

    let block_info = BlockInfo {
        height: trade_state.expire_height + 1,
        time: from.time,
        chain_id: from.chain_id.clone(),
    };
    vars.router.set_block(block_info);

    let release_response = vars.router.execute_contract(
        vars.trade_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Release {},
        &[],
    );
    assert!(release_response.is_err());

    let old_trade_contract_balance = query_ust_balance(vars, trade_contract_addr.clone());
    assert_ne!(old_trade_contract_balance, Uint128::zero());

    let old_trade_owner_balance = query_ust_balance(vars, vars.trade_owner.clone());

    let refund_response = vars.router.execute_contract(
        vars.trade_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Refund {},
        &[],
    );
    assert!(refund_response.is_ok());

    let trade_contract_balance = query_ust_balance(vars, trade_contract_addr.clone());
    assert_eq!(trade_contract_balance, terra_tax);

    let new_trader_owner_balance = query_ust_balance(vars, vars.trade_owner.clone());
    assert_eq!(
        new_trader_owner_balance,
        (old_trade_contract_balance + old_trade_owner_balance)
            .checked_sub(terra_tax)
            .unwrap(),
    );
}

#[test]
fn test_errors() {
    let vars = &mut init(OfferType::Buy);
    let initial_trade_owner_balance = Uint128::new(2_000_000);

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);

    let init_funds = [coin(*&initial_trade_owner_balance.u128(), vars.ust_denom)].to_vec();
    vars.router
        .set_bank_balance(&vars.trade_owner, init_funds)
        .unwrap();

    //Create Trade
    let create_trade_result = create_trade(vars, trade_amount);
    assert!(create_trade_result.is_ok());

    //Assert that the state is EscrowFunded
    let trade_contract_addr = create_trade_result.unwrap();
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::EscrowFunded);

    //Try to release as the offer (and not the trade) owner.
    let release_response = vars.router.execute_contract(
        vars.offer_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Release {},
        &[],
    );
    assert!(release_response.is_err());
    assert_eq!(release_response.err().unwrap(), "Unauthorized.");
}

#[test]
fn test_sell_cw20_token() {
    let vars = &mut init(OfferType::Buy);

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);
    vars.router
        .set_bank_balance(&vars.trade_owner, vec![])
        .unwrap();

    // set up cw20 contract with some tokens
    let cw20_id = vars.router.store_code(contract_cw20());
    let cw20_instantiate_msg = cw20_base::msg::InstantiateMsg {
        name: "Mirrored COIN".to_string(),
        symbol: "mcoin".to_string(),
        decimals: 2,
        initial_balances: vec![Cw20Coin {
            address: vars.trade_owner.to_string(),
            amount: Uint128::new(500000),
        }],
        mint: None,
    };
    let mcoin_addr = vars
        .router
        .instantiate_contract(
            cw20_id,
            vars.trade_owner.clone(),
            &cw20_instantiate_msg,
            &[],
            "CASH",
        )
        .unwrap();
    let mcoin = Cw20Contract(mcoin_addr.clone());
    let owner_balance = mcoin
        .balance(&vars.router, vars.trade_owner.to_string())
        .unwrap();
    assert_eq!(owner_balance, Uint128::new(500000));
    vars.init_terraswap_mock(mcoin_addr.clone());
    vars.router
        .set_bank_balance(
            &vars.terraswap_pair_addr.clone().unwrap(),
            vec![Coin {
                denom: "uusd".to_string(),
                amount: Uint128::new(1_000_000_000_000),
            }],
        )
        .unwrap();

    //Create Trade
    let create_trade_result = create_trade_with_funds(
        vars,
        trade_amount,
        false,
        vars.terraswap_factory_addr.clone(),
        vars.ust_denom,
    );
    assert!(create_trade_result.is_ok());

    //Assert that the Trade state is Created
    let trade_contract_addr = create_trade_result.unwrap();
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::Created);

    //Send mCOIN to trade.
    let send_mcoin_msg = Cw20ExecuteMsg::Send {
        contract: trade_contract_addr.to_string(),
        amount: Uint128::new(500000),
        msg: Some(to_binary("").unwrap()),
    };
    vars.router
        .execute_contract(
            vars.trade_owner.clone(),
            mcoin_addr.clone(),
            &send_mcoin_msg,
            &[],
        )
        .unwrap();

    let owner_balance = mcoin
        .balance(&vars.router, vars.trade_owner.to_string())
        .unwrap();
    assert_eq!(owner_balance, Uint128::new(0));

    let trade_contract_ust_balance = query_ust_balance(vars, trade_contract_addr.clone());
    assert_eq!(trade_contract_ust_balance, Uint128::new(500000));

    vars.router
        .execute_contract(
            vars.trade_owner.clone(),
            trade_contract_addr.clone(),
            &ExecuteMsg::FundEscrow,
            &[],
        )
        .unwrap();
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::EscrowFunded);
}

#[test]
fn test_buy_cw20_token() {
    let vars = &mut init(OfferType::Sell);

    let initial_offer_owner_balance = Uint128::new(1_000_000);
    let init_funds = [coin(*&initial_offer_owner_balance.u128(), vars.ust_denom)].to_vec();

    let offer_state: Offer = query_offer_contract(vars, &offer::msg::QueryMsg::LoadOffer { id: 1 });
    let trade_amount = Uint128::from(offer_state.max_amount);

    vars.router
        .set_bank_balance(&vars.offer_owner, init_funds)
        .unwrap();

    // set up cw20 contract with some tokens
    let cw20_id = vars.router.store_code(contract_cw20());
    let cw20_instantiate_msg = cw20_base::msg::InstantiateMsg {
        name: "Mirrored COIN".to_string(),
        symbol: "mcoin".to_string(),
        decimals: 2,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter: vars.offer_owner.to_string(),
            cap: None,
        }),
    };
    let mcoin_addr = vars
        .router
        .instantiate_contract(
            cw20_id,
            vars.trade_owner.clone(),
            &cw20_instantiate_msg,
            &[],
            "CASH",
        )
        .unwrap();
    let mcoin = Cw20Contract(mcoin_addr.clone());
    vars.init_terraswap_mock(mcoin_addr.clone());

    let cw20_mint_to_pair_msg = cw20_base::msg::ExecuteMsg::Mint {
        recipient: vars.terraswap_pair_addr.clone().unwrap().to_string(),
        amount: Uint128::new(1_000_000),
    };
    let mint_response = vars.router.execute_contract(
        vars.offer_owner.clone(),
        mcoin_addr.clone(),
        &cw20_mint_to_pair_msg,
        &[],
    );
    assert!(mint_response.is_ok());

    let trade_pair_cw20_balance = mcoin
        .balance(
            &vars.router,
            vars.terraswap_pair_addr.clone().unwrap().to_string(),
        )
        .unwrap();
    assert_eq!(trade_pair_cw20_balance, Uint128::new(1000000));

    //Create Trade
    let create_trade_result = create_trade_with_funds(
        vars,
        trade_amount,
        false,
        vars.terraswap_factory_addr.clone(),
        mcoin_addr.clone().as_str(),
    );
    assert!(create_trade_result.is_ok());

    //Assert that the Trade state is Created
    let trade_contract_addr = create_trade_result.unwrap();
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::Created);

    let fund_escrow_response = vars.router.execute_contract(
        vars.offer_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::FundEscrow {},
        &[Coin {
            denom: vars.ust_denom.to_string(),
            amount: offer_state.max_amount,
        }],
    );
    assert!(fund_escrow_response.is_ok());

    //Assert that the Trade state is EscrowFunded
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::EscrowFunded);

    let release_response = vars.router.execute_contract(
        vars.offer_owner.clone(),
        trade_contract_addr.clone(),
        &ExecuteMsg::Release,
        &[],
    );
    assert!(release_response.is_ok());

    //Assert that the Trade state is Closed
    let trade_state: State =
        query_trade_contract(vars, trade_contract_addr.clone(), &QueryMsg::Config {});
    assert_eq!(trade_state.state, TradeState::Closed);

    let trade_owner_cw20_balance = mcoin
        .balance(&vars.router, vars.trade_owner.clone())
        .unwrap();
    assert!(trade_owner_cw20_balance.gt(&Uint128::zero()));
}
