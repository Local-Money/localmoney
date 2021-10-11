#[allow(dead_code)]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::currencies::FiatCurrency;
use crate::factory::Config as FactoryConfig;
use crate::governance::Config as GovConfig;
use crate::offer::{Config as OfferConfig, Offer, OfferState, OfferType, TradeInfo};
use crate::trade::{State as TradeState, TradeState as TradeTradeState};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, Coin, ContractResult, Decimal, OwnedDeps, Querier,
    QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};
use cw20::BalanceResponse;
use std::collections::HashMap;
use terra_cosmwasm::{TaxCapResponse, TaxRateResponse, TerraQuery, TerraQueryWrapper, TerraRoute};
use terraswap::asset::{AssetInfo, PairInfo};

/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies(
    contract_balance: &[Coin],
    offer: Option<Offer>,
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier = WasmMockQuerier::new(
        MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]),
        offer,
    );

    OwnedDeps {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<TerraQueryWrapper>,
    token_querier: TokenQuerier,
    tax_querier: TaxQuerier,
    terraswap_factory_querier: TerraswapFactoryQuerier,
    offer: Option<Offer>,
}

#[derive(Clone, Default)]
pub struct TokenQuerier {
    // this lets us iterate over all pairs that match the first string
    balances: HashMap<String, HashMap<String, Uint128>>,
}

impl TokenQuerier {
    pub fn new(balances: &[(&String, &[(&String, &Uint128)])]) -> Self {
        TokenQuerier {
            balances: balances_to_map(balances),
        }
    }
}

pub(crate) fn balances_to_map(
    balances: &[(&String, &[(&String, &Uint128)])],
) -> HashMap<String, HashMap<String, Uint128>> {
    let mut balances_map: HashMap<String, HashMap<String, Uint128>> = HashMap::new();
    for (contract_addr, balances) in balances.iter() {
        let mut contract_balances_map: HashMap<String, Uint128> = HashMap::new();
        for (addr, balance) in balances.iter() {
            contract_balances_map.insert(addr.to_string(), **balance);
        }

        balances_map.insert(contract_addr.to_string(), contract_balances_map);
    }
    balances_map
}

#[derive(Clone, Default)]
pub struct TaxQuerier {
    rate: Decimal,
    // this lets us iterate over all pairs that match the first string
    caps: HashMap<String, Uint128>,
}

impl TaxQuerier {
    pub fn new(rate: Decimal, caps: &[(&String, &Uint128)]) -> Self {
        TaxQuerier {
            rate,
            caps: caps_to_map(caps),
        }
    }
}

pub(crate) fn caps_to_map(caps: &[(&String, &Uint128)]) -> HashMap<String, Uint128> {
    let mut owner_map: HashMap<String, Uint128> = HashMap::new();
    for (denom, cap) in caps.iter() {
        owner_map.insert(denom.to_string(), **cap);
    }
    owner_map
}

#[derive(Clone, Default)]
pub struct TerraswapFactoryQuerier {
    pairs: HashMap<String, String>,
}

impl TerraswapFactoryQuerier {
    pub fn new(pairs: &[(&String, &String)]) -> Self {
        TerraswapFactoryQuerier {
            pairs: pairs_to_map(pairs),
        }
    }
}

pub(crate) fn pairs_to_map(pairs: &[(&String, &String)]) -> HashMap<String, String> {
    let mut pairs_map: HashMap<String, String> = HashMap::new();
    for (key, pair) in pairs.iter() {
        pairs_map.insert(key.to_string(), pair.to_string());
    }
    pairs_map
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Pair { asset_infos: [AssetInfo; 2] },
    Balance { address: String },
    Offer { id: u64 },
    Config {},
    LoadTrades { maker: String },
    TradeInfo { maker: String, trade: String },
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Custom(TerraQueryWrapper { route, query_data }) => {
                if route == &TerraRoute::Treasury {
                    match query_data {
                        TerraQuery::TaxRate {} => {
                            let res = TaxRateResponse {
                                rate: self.tax_querier.rate,
                            };
                            SystemResult::Ok(ContractResult::from(to_binary(&res)))
                        }
                        TerraQuery::TaxCap { denom } => {
                            let cap = self
                                .tax_querier
                                .caps
                                .get(denom)
                                .copied()
                                .unwrap_or_default();
                            let res = TaxCapResponse { cap };
                            SystemResult::Ok(ContractResult::from(to_binary(&res)))
                        }
                        _ => panic!("DO NOT ENTER HERE"),
                    }
                } else {
                    panic!("DO NOT ENTER HERE")
                }
            }
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => match from_binary(&msg)
                .unwrap()
            {
                QueryMsg::Pair { asset_infos } => {
                    let key = asset_infos[0].to_string() + asset_infos[1].to_string().as_str();
                    match self.terraswap_factory_querier.pairs.get(&key) {
                        Some(v) => SystemResult::Ok(ContractResult::from(to_binary(&PairInfo {
                            contract_addr: v.to_string(),
                            liquidity_token: "liquidity".to_string(),
                            asset_infos: [
                                AssetInfo::NativeToken {
                                    denom: "uusd".to_string(),
                                },
                                AssetInfo::NativeToken {
                                    denom: "uusd".to_string(),
                                },
                            ],
                        }))),
                        None => SystemResult::Err(SystemError::InvalidRequest {
                            error: "No pair info exists".to_string(),
                            request: msg.as_slice().into(),
                        }),
                    }
                }
                QueryMsg::Balance { address } => {
                    let balances: &HashMap<String, Uint128> =
                        match self.token_querier.balances.get(contract_addr) {
                            Some(balances) => balances,
                            None => {
                                return SystemResult::Err(SystemError::InvalidRequest {
                                    error: format!(
                                        "No balance info exists for the contract {}",
                                        contract_addr
                                    ),
                                    request: msg.as_slice().into(),
                                })
                            }
                        };

                    let balance = match balances.get(&address) {
                        Some(v) => *v,
                        None => {
                            return SystemResult::Ok(ContractResult::Ok(
                                to_binary(&BalanceResponse {
                                    balance: Uint128::zero(),
                                })
                                .unwrap(),
                            ));
                        }
                    };

                    SystemResult::Ok(ContractResult::Ok(
                        to_binary(&BalanceResponse { balance }).unwrap(),
                    ))
                }
                QueryMsg::Offer { id } => {
                    let offer = self.offer.clone().unwrap_or(Offer {
                        id,
                        owner: Addr::unchecked("offer-owner"),
                        offer_type: OfferType::Buy,
                        fiat_currency: FiatCurrency::COP,
                        min_amount: Uint128::new(1_000_000u128),
                        max_amount: Uint128::new(500_000_000u128),
                        state: OfferState::Active,
                    });
                    SystemResult::Ok(ContractResult::from(to_binary(&offer)))
                }
                //TODO: This will fail if other Config query is made from tests,
                // we need a reliable way to check which config is being queried.
                QueryMsg::Config {} => {
                    if contract_addr.contains("factory") {
                        SystemResult::Ok(ContractResult::from(to_binary(&FactoryConfig {
                            trade_code_id: 0,
                            token_addr: Addr::unchecked("local"),
                            local_ust_pool_addr: Addr::unchecked("local-ust"),
                            gov_addr: Addr::unchecked("gov"),
                            offers_addr: Addr::unchecked("offers"),
                            fee_collector_addr: Addr::unchecked("fee-collector"),
                            trading_incentives_addr: Addr::unchecked("trading-incentives"),
                        })))
                    } else if contract_addr.contains("gov") {
                        SystemResult::Ok(ContractResult::from(to_binary(&GovConfig {
                            factory_addr: Addr::unchecked("factory"),
                        })))
                    } else {
                        let offer_config = OfferConfig {
                            factory_addr: Addr::unchecked("factory"),
                        };
                        SystemResult::Ok(ContractResult::from(to_binary(&offer_config)))
                    }
                }
                QueryMsg::LoadTrades { .. } => {
                    SystemResult::Ok(ContractResult::from(to_binary(&vec!["trade0000"])))
                }
                QueryMsg::TradeInfo { trade: _, maker } => {
                    SystemResult::Ok(ContractResult::from(to_binary(&TradeInfo {
                        trade: TradeState {
                            factory_addr: Addr::unchecked("factory"),
                            recipient: Addr::unchecked("taker"),
                            sender: Addr::unchecked(maker),
                            offer_id: 1,
                            offer_contract: Addr::unchecked("offer"),
                            state: TradeTradeState::Closed,
                            expire_height: 0,
                            ust_amount: Uint128::new(1_000_000u128),
                        },
                        offer: Offer {
                            id: 1,
                            owner: Addr::unchecked("offer-owner"),
                            offer_type: OfferType::Buy,
                            fiat_currency: FiatCurrency::COP,
                            min_amount: Uint128::new(1_000_000u128),
                            max_amount: Uint128::new(500_000_000u128),
                            state: OfferState::Active,
                        },
                    })))
                }
            },
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<TerraQueryWrapper>, offer: Option<Offer>) -> Self {
        WasmMockQuerier {
            base,
            token_querier: TokenQuerier::default(),
            tax_querier: TaxQuerier::default(),
            terraswap_factory_querier: TerraswapFactoryQuerier::default(),
            offer,
        }
    }

    // configure the mint whitelist mock querier
    pub fn with_token_balances(&mut self, balances: &[(&String, &[(&String, &Uint128)])]) {
        self.token_querier = TokenQuerier::new(balances);
    }

    // configure the token owner mock querier
    pub fn with_tax(&mut self, rate: Decimal, caps: &[(&String, &Uint128)]) {
        self.tax_querier = TaxQuerier::new(rate, caps);
    }

    // configure the terraswap pair
    pub fn with_terraswap_pairs(&mut self, pairs: &[(&String, &String)]) {
        self.terraswap_factory_querier = TerraswapFactoryQuerier::new(pairs);
    }

    pub fn update_balance(
        &mut self,
        addr: impl Into<String>,
        balance: Vec<Coin>,
    ) -> Option<Vec<Coin>> {
        self.base.update_balance(addr, balance)
    }
}
