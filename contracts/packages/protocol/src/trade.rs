use std::fmt::{self, Display};
use std::ops::{Add, Mul};

use cosmwasm_std::{
    Addr, BlockInfo, Coin, CustomQuery, Decimal, Deps, Env, MessageInfo, Order, StdResult, Storage,
    Uint128, Uint256,
};
use cw20::Denom;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Item, Map, MultiIndex, UniqueIndex};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::currencies::FiatCurrency;
use crate::guards::assert_range_0_to_99;
use crate::offer::Arbitrator;
use crate::profile::Profile;

pub const DENOM_CONVERSION_ROUTE: Map<&str, Vec<ConversionRoute>> =
    Map::new("denom_conversion_route");
pub const DENOM_CONVERSION_STEP: Item<ConversionStep> = Item::new("denom_conversion_step");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create(NewTrade),
    AcceptRequest {
        trade_id: u64,
        maker_contact: String,
    },
    FundEscrow {
        trade_id: u64,
        maker_contact: Option<String>,
    },
    RefundEscrow {
        trade_id: u64,
    },
    ReleaseEscrow {
        trade_id: u64,
    },
    DisputeEscrow {
        trade_id: u64,
        buyer_contact: String,
        seller_contact: String,
    },
    FiatDeposited {
        trade_id: u64,
    },
    CancelRequest {
        trade_id: u64,
    },
    NewArbitrator {
        arbitrator: Addr,
        fiat: FiatCurrency,
        encryption_key: String,
    },
    DeleteArbitrator {
        arbitrator: Addr,
        fiat: FiatCurrency,
    },
    SettleDispute {
        trade_id: u64,
        winner: Addr,
    },
    RegisterHub {},
    RegisterConversionRouteForDenom {
        denom: Denom,
        route: Vec<ConversionRoute>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Trade {
        id: u64,
    },
    Trades {
        user: Addr,
        role: TraderRole,
        limit: u32,
        last: Option<u64>,
    },
    Arbitrator {
        arbitrator: Addr,
    },
    Arbitrators {},
    ArbitratorsFiat {
        fiat: FiatCurrency,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FeeInfo {
    pub burn_amount: Uint128,
    pub chain_amount: Uint128,
    pub warchest_amount: Uint128,
}

impl Display for FeeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "burn_amount: {}, chain_amount: {}, warchest_amount: {}",
            self.burn_amount, self.chain_amount, self.warchest_amount
        )
    }
}

impl FeeInfo {
    pub fn total_fees(&self) -> Uint128 {
        self.burn_amount
            .add(self.chain_amount)
            .add(self.warchest_amount)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConversionRoute {
    pub pool: Addr,
    pub ask_asset: Denom,
    pub offer_asset: Denom,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConversionStep {
    pub trade_denom: Denom,
    pub step_previous_balance: Coin,
    pub step: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TraderRole {
    Trader,
    Arbitrator,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NewTrade {
    pub offer_id: u64,
    pub amount: Uint128,
    pub taker: Addr,
    pub profile_taker_contact: String,
    pub profile_taker_encryption_key: String,
    pub taker_contact: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SwapMsg {
    pub swap: Swap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Swap {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TradeState {
    RequestCreated,
    RequestCanceled,
    RequestExpired,
    RequestAccepted,
    EscrowFunded,
    EscrowCanceled,
    EscrowRefunded,
    FiatDeposited,
    EscrowReleased,
    EscrowDisputed,
    SettledForMaker,
    SettledForTaker,
}

impl fmt::Display for TradeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Trade {
    pub id: u64,
    pub addr: Addr,
    pub buyer: Addr,
    pub buyer_contact: Option<String>,
    pub seller: Addr,
    pub seller_contact: Option<String>,
    pub arbitrator: Addr,
    pub arbitrator_buyer_contact: Option<String>,
    pub arbitrator_seller_contact: Option<String>,
    pub offer_contract: Addr,
    pub offer_id: u64,
    pub created_at: u64,
    pub expires_at: u64,
    pub enables_dispute_at: Option<u64>,
    pub denom: Denom,
    pub amount: Uint128,
    pub fiat: FiatCurrency,
    pub denom_fiat_price: Uint256,
    pub state_history: Vec<TradeStateItem>,
    state: TradeState,
}

impl Trade {
    pub fn new(
        id: u64,
        addr: Addr,
        buyer: Addr,
        seller: Addr,
        seller_contact: Option<String>,
        buyer_contact: Option<String>,
        arbitrator: Addr,
        offer_contract: Addr,
        offer_id: u64,
        created_at: u64,
        expires_at: u64,
        denom: Denom,
        amount: Uint128,
        fiat: FiatCurrency,
        denom_fiat_price: Uint256,
        state_history: Vec<TradeStateItem>,
    ) -> Trade {
        return Trade {
            id,
            addr,
            buyer,
            seller,
            seller_contact,
            arbitrator_buyer_contact: None,
            arbitrator_seller_contact: None,
            buyer_contact,
            arbitrator,
            offer_contract,
            offer_id,
            created_at,
            expires_at,
            enables_dispute_at: None,
            denom,
            amount,
            fiat,
            denom_fiat_price,
            state_history,
            state: TradeState::RequestCreated,
        };
    }

    pub fn get_state(&self) -> TradeState {
        return self.state.clone();
    }

    pub fn request_expired(&self, block_time: u64) -> bool {
        return self.expires_at.ne(&0) && block_time > self.expires_at;
    }

    pub fn set_state(&mut self, new_state: TradeState, env: &Env, info: &MessageInfo) {
        // if the escrow is canceled or fiat is already deposited, the trade can no longer expire
        if vec![
            TradeState::RequestCanceled,
            TradeState::EscrowCanceled,
            TradeState::FiatDeposited,
        ]
        .contains(&new_state)
        {
            self.expires_at = 0;
        }

        let block: BlockInfo = env.block.clone();
        self.state = new_state;
        let new_trade_state = TradeStateItem {
            actor: info.sender.clone(),
            state: self.get_state(),
            timestamp: block.time.seconds(),
        };
        self.state_history.push(new_trade_state);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeResponse {
    pub id: u64,
    pub addr: Addr,
    pub buyer: Addr,
    pub buyer_contact: Option<String>,
    pub buyer_encryption_key: Option<String>,
    pub seller: Addr,
    pub seller_contact: Option<String>,
    pub seller_encryption_key: Option<String>,
    pub arbitrator: Option<Addr>,
    pub arbitrator_encryption_key: Option<String>,
    pub arbitrator_seller_contact: Option<String>,
    pub arbitrator_buyer_contact: Option<String>,
    pub offer_contract: Addr,
    pub offer_id: u64,
    pub created_at: u64,
    pub expires_at: u64,
    pub enables_dispute_at: Option<u64>,
    pub denom: Denom,
    pub amount: Uint128,
    pub fiat: FiatCurrency,
    pub denom_fiat_price: Uint256,
    pub state_history: Vec<TradeStateItem>,
    pub state: TradeState,
}

impl TradeResponse {
    pub fn map(
        trade: Trade,
        buyer_profile: Profile,
        seller_profile: Profile,
        arbitrator_profile: Profile,
        block_time: u64,
    ) -> TradeResponse {
        let trade_states = vec![
            TradeState::EscrowDisputed,
            TradeState::SettledForMaker,
            TradeState::SettledForTaker,
        ];
        let state = if trade.request_expired(block_time) {
            TradeState::RequestExpired
        } else {
            trade.get_state()
        };

        let arbitrator_address: Option<Addr> = if trade_states.contains(&state) {
            Some(trade.arbitrator)
        } else {
            None
        };

        let arbitrator_encryption_key: Option<String> = if state.eq(&TradeState::FiatDeposited) {
            arbitrator_profile.encryption_key.clone()
        } else {
            None
        };

        TradeResponse {
            id: trade.id,
            addr: trade.addr,
            buyer: trade.buyer,
            buyer_contact: trade.buyer_contact,
            buyer_encryption_key: buyer_profile.encryption_key,
            seller: trade.seller,
            seller_contact: trade.seller_contact,
            seller_encryption_key: seller_profile.encryption_key,
            arbitrator: arbitrator_address,
            arbitrator_encryption_key,
            arbitrator_seller_contact: trade.arbitrator_seller_contact,
            arbitrator_buyer_contact: trade.arbitrator_buyer_contact,
            offer_contract: trade.offer_contract,
            offer_id: trade.offer_id,
            created_at: trade.created_at,
            expires_at: trade.expires_at,
            enables_dispute_at: trade.enables_dispute_at,
            denom: trade.denom,
            amount: trade.amount,
            fiat: trade.fiat,
            denom_fiat_price: trade.denom_fiat_price,
            state_history: trade.state_history,
            state,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeStateItem {
    pub actor: Addr,
    pub state: TradeState,
    pub timestamp: u64,
}

pub struct TradeModel<'a> {
    pub trade: Trade,
    pub storage: &'a mut dyn Storage,
}

impl TradeModel<'_> {
    pub fn size(storage: &mut dyn Storage) -> usize {
        trades()
            .range(storage, None, None, Order::Descending)
            .count()
    }

    pub fn store(storage: &mut dyn Storage, trade: &Trade) -> StdResult<()> {
        trades().save(storage, trade.id, trade)
    }

    pub fn from_store(storage: &dyn Storage, id: u64) -> Trade {
        trades().may_load(storage, id).unwrap_or_default().unwrap()
    }

    pub fn create(storage: &mut dyn Storage, trade: Trade) -> TradeModel {
        TradeModel::store(storage, &trade).unwrap();
        TradeModel { trade, storage }
    }

    pub fn save<'a>(self) -> Trade {
        TradeModel::store(self.storage, &self.trade).unwrap();
        self.trade
    }

    pub fn may_load<'a>(storage: &'a mut dyn Storage, id: u64) -> TradeModel<'a> {
        let trade_model = TradeModel {
            trade: TradeModel::from_store(storage, id),
            storage,
        };
        return trade_model;
    }

    pub fn trades_by_trader(
        storage: &dyn Storage,
        trader: String,
        limit: usize,
        last: Option<u64>,
    ) -> StdResult<Vec<Trade>> {
        let range_from = last.map(Bound::exclusive);

        let result = trades()
            .idx
            .collection
            .range(storage, None, range_from, Order::Descending)
            .filter_map(|item| {
                item.and_then(|(_, trade)| {
                    if trade.seller.eq(&trader) || trade.buyer.eq(&trader) {
                        Ok(Some(trade))
                    } else {
                        Ok(None)
                    }
                })
                .unwrap()
            })
            .take(limit)
            .collect();

        Ok(result)
    }

    pub fn trades_by_arbitrator(
        storage: &dyn Storage,
        arbitrator: String,
        limit: usize,
        last: Option<u64>,
    ) -> StdResult<Vec<Trade>> {
        let range_from = last.map(Bound::exclusive);

        let trade_states = vec![
            TradeState::EscrowDisputed,
            TradeState::SettledForMaker,
            TradeState::SettledForTaker,
        ];

        let result = trades()
            .idx
            .arbitrator
            .prefix(arbitrator)
            .range(storage, None, range_from, Order::Descending)
            .take(limit)
            .filter_map(|item| {
                item.and_then(|(_, trade)| {
                    if trade_states.contains(&trade.get_state()) {
                        Ok(Some(trade))
                    } else {
                        Ok(None)
                    }
                })
                .unwrap()
            })
            .collect();

        Ok(result)
    }
}

pub struct TradeIndexes<'a> {
    // pk goes to second tuple element
    pub collection: UniqueIndex<'a, u64, Trade, u64>,
    pub arbitrator: MultiIndex<'a, String, Trade, u64>,
}

impl<'a> IndexList<Trade> for TradeIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Trade>> + '_> {
        let v: Vec<&dyn Index<Trade>> = vec![&self.collection, &self.arbitrator];
        Box::new(v.into_iter())
    }
}

pub fn trades<'a>() -> IndexedMap<'a, u64, Trade, TradeIndexes<'a>> {
    let pk_namespace = "trades_v0_4_2";
    let indexes = TradeIndexes {
        collection: UniqueIndex::new(|t| t.id, "trades__collection"),
        arbitrator: MultiIndex::new(
            |t| t.arbitrator.to_string(),
            pk_namespace,
            "trades__arbitrator",
        ),
    };
    IndexedMap::new(pk_namespace, indexes)
}

// Arbitrator
pub struct ArbitratorModel {}

impl ArbitratorModel {
    pub fn create_arbitrator(storage: &mut dyn Storage, arbitrator: Arbitrator) {
        let index = arbitrator.arbitrator.clone().to_string() + &arbitrator.fiat.to_string();
        arbitrators().save(storage, &index, &arbitrator).unwrap();
    }

    pub fn query_arbitrator(storage: &dyn Storage, arbitrator: Addr) -> StdResult<Vec<Arbitrator>> {
        let result = arbitrators()
            .idx
            .arbitrator
            .prefix(arbitrator)
            .range(storage, None, None, Order::Descending)
            .flat_map(|item| item.and_then(|(_, arbitrator)| Ok(arbitrator)))
            .collect();

        Ok(result)
    }

    pub fn query_arbitrator_fiat(
        storage: &dyn Storage,
        arbitrator: Addr,
        fiat: FiatCurrency,
    ) -> StdResult<Arbitrator> {
        let result = arbitrators()
            .idx
            .arbitrator
            .prefix(arbitrator)
            .range(storage, None, None, Order::Descending)
            .find_map(|item| {
                item.and_then(|(_, arb)| {
                    if arb.fiat.eq(&fiat) {
                        Ok(Some(arb))
                    } else {
                        Ok(None)
                    }
                })
                .unwrap()
            })
            .unwrap();

        Ok(result)
    }

    pub fn query_arbitrators(storage: &dyn Storage) -> StdResult<Vec<Arbitrator>> {
        let result = arbitrators()
            .range(storage, None, None, Order::Descending)
            .flat_map(|item| item.and_then(|(_, arbitrator)| Ok(arbitrator)))
            .collect();

        Ok(result)
    }

    pub fn query_arbitrators_fiat(
        storage: &dyn Storage,
        fiat: FiatCurrency,
    ) -> StdResult<Vec<Arbitrator>> {
        let result: Vec<Arbitrator> = arbitrators()
            .idx
            .fiat
            .prefix(fiat.clone().to_string())
            .range(storage, None, None, Order::Descending)
            .take(10)
            .flat_map(|item| item.and_then(|(_, arbitrator)| Ok(arbitrator)))
            .collect();

        Ok(result)
    }

    pub fn get_arbitrator_random<T: CustomQuery>(
        deps: Deps<T>,
        random_value: usize,
        fiat: FiatCurrency,
    ) -> Arbitrator {
        assert_range_0_to_99(random_value).unwrap();
        let storage = deps.storage;
        let result: Vec<Arbitrator> = arbitrators()
            .idx
            .fiat
            .prefix(fiat.to_string())
            .range(storage, None, None, Order::Descending)
            .take(10)
            .flat_map(|item| item.and_then(|(_, arbitrator)| Ok(arbitrator)))
            .collect();
        let arbitrator_count = result.len();

        // Random range: 0..99
        // Mapped range: 0..result.len()-1
        // Formula is:
        // RandomValue * (MaxMappedRange + 1) / (MaxRandomRange + 1)
        let random_index = random_value * arbitrator_count / (99 + 1);
        result[random_index].clone()
    }
}

pub fn arbitrators<'a>() -> IndexedMap<'a, &'a str, Arbitrator, ArbitratorIndexes<'a>> {
    let arbitrators_pk_namespace = "arbitrators_v0_3_0";
    let indexes = ArbitratorIndexes {
        arbitrator: MultiIndex::new(
            |d: &Arbitrator| d.arbitrator.clone(),
            arbitrators_pk_namespace,
            "arbitrators__arbitrator",
        ),
        fiat: MultiIndex::new(
            |d: &Arbitrator| d.fiat.clone().to_string(),
            arbitrators_pk_namespace,
            "arbitrators__asset",
        ),
    };
    IndexedMap::new(&arbitrators_pk_namespace, indexes)
}

pub struct ArbitratorIndexes<'a> {
    // pk goes to second tuple element
    pub arbitrator: MultiIndex<'a, Addr, Arbitrator, Vec<u8>>,
    pub fiat: MultiIndex<'a, String, Arbitrator, Vec<u8>>,
}

impl<'a> IndexList<Arbitrator> for ArbitratorIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Arbitrator>> + '_> {
        let v: Vec<&dyn Index<Arbitrator>> = vec![&self.arbitrator, &self.fiat];
        Box::new(v.into_iter())
    }
}

pub fn calc_denom_fiat_price(offer_rate: Uint128, denom_fiat_price: Uint256) -> Uint256 {
    let hundred = Uint128::new(100u128);
    let offer_rate = Decimal::from_ratio(offer_rate.clone(), hundred);
    let offer_rate = Uint256::from(hundred.mul(offer_rate)); //% 100
    denom_fiat_price
        .checked_mul(offer_rate)
        .unwrap_or(Uint256::zero())
        .checked_div(Uint256::from(hundred))
        .unwrap_or(Uint256::zero())
}
