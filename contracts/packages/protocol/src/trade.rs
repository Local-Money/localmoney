use std::fmt::{self};

use cosmwasm_std::{Addr, Order, StdResult, Storage, Uint128};
use cw20::Denom;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::currencies::FiatCurrency;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create(NewTrade),
    AcceptRequest { trade_id: String },
    FundEscrow { trade_id: String },
    RefundEscrow { trade_id: String },
    ReleaseEscrow { trade_id: String },
    DisputeEscrow { trade_id: String },
    FiatDeposited { trade_id: String },
    CancelRequest { trade_id: String },
    RegisterHub {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Trade {
        id: String,
    },
    Trades {
        user: Addr,
        state: Option<TradeState>,
        role: TraderRole,
        last_value: Option<String>,
        limit: u32,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TraderRole {
    Seller,
    Buyer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NewTrade {
    pub offer_id: String,
    pub amount: Uint128,
    pub taker: Addr,
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
    RequestAccepted,
    RequestCanceled,
    RequestExpired,
    EscrowFunded,
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
    pub id: String,
    pub addr: Addr,
    pub buyer: Addr,
    pub seller: Addr,
    pub arbitrator: Option<Addr>,
    pub offer_contract: Addr,
    pub offer_id: String,
    pub created_at: u64,
    pub denom: Denom,
    pub amount: Uint128,
    pub state: TradeState,
    pub state_history: Vec<TradeStateItem>,
    pub asset: FiatCurrency,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeStateItem {
    pub actor: Addr,
    pub state: TradeState,
    pub timestamp: String,
}

pub struct TradeModel<'a> {
    pub trade: Trade,
    pub storage: &'a mut dyn Storage,
}

impl TradeModel<'_> {
    pub fn store(storage: &mut dyn Storage, trade: &Trade) -> StdResult<()> {
        trades().save(storage, trade.id.to_string(), trade)
    }

    pub fn from_store(storage: &dyn Storage, id: &String) -> Trade {
        trades()
            .may_load(storage, id.to_string())
            .unwrap_or_default()
            .unwrap()
    }

    pub fn create(storage: &mut dyn Storage, trade: Trade) -> TradeModel {
        TradeModel::store(storage, &trade).unwrap();
        TradeModel { trade, storage }
    }

    pub fn save<'a>(self) -> Trade {
        TradeModel::store(self.storage, &self.trade).unwrap();
        self.trade
    }

    pub fn may_load<'a>(storage: &'a mut dyn Storage, id: &String) -> TradeModel<'a> {
        let trade_model = TradeModel {
            trade: TradeModel::from_store(storage, &id),
            storage,
        };
        return trade_model;
    }

    pub fn trades_by_buyer(
        storage: &dyn Storage,
        buyer: String,
        last_value: Option<String>,
        limit: u32,
    ) -> StdResult<Vec<Trade>> {
        let range_from = match last_value {
            Some(thing) => Some(Bound::exclusive(thing)),
            None => None,
        };

        let result = trades()
            .idx
            .buyer
            .prefix(buyer.to_string())
            .range(storage, range_from, None, Order::Descending)
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, trade)| Ok(trade)))
            .collect();

        Ok(result)
    }

    pub fn trades_by_seller(
        storage: &dyn Storage,
        seller: String,
        last_value: Option<String>,
        limit: u32,
    ) -> StdResult<Vec<Trade>> {
        let range_from = match last_value {
            Some(thing) => Some(Bound::exclusive(thing)),
            None => None,
        };

        let result = trades()
            .idx
            .seller
            .prefix(seller.to_string())
            .range(storage, range_from, None, Order::Descending)
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, trade)| Ok(trade)))
            .collect();

        Ok(result)
    }
}

pub struct TradeIndexes<'a> {
    // pk goes to second tuple element
    pub buyer: MultiIndex<'a, String, Trade, String>,
    pub seller: MultiIndex<'a, String, Trade, String>,
}

impl<'a> IndexList<Trade> for TradeIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Trade>> + '_> {
        let v: Vec<&dyn Index<Trade>> = vec![&self.buyer, &self.seller];
        Box::new(v.into_iter())
    }
}

pub fn trades<'a>() -> IndexedMap<'a, String, Trade, TradeIndexes<'a>> {
    let indexes = TradeIndexes {
        buyer: MultiIndex::new(|t| t.buyer.to_string(), "trades", "trades__buyer"),
        seller: MultiIndex::new(|t| t.seller.to_string(), "trades", "trades__seller"),
    };
    IndexedMap::new("trades", indexes)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
