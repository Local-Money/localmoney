use std::fmt::{self};

use cosmwasm_std::{Addr, BlockInfo, Env, MessageInfo, Order, StdResult, Storage, Uint128};
use cw20::Denom;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::currencies::FiatCurrency;
use crate::offer::Arbitrator;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create(NewTrade),
    AcceptRequest {
        trade_id: String,
        maker_contact: String,
    },
    FundEscrow {
        trade_id: String,
        maker_contact: Option<String>,
    },
    RefundEscrow {
        trade_id: String,
    },
    ReleaseEscrow {
        trade_id: String,
    },
    DisputeEscrow {
        trade_id: String,
    },
    FiatDeposited {
        trade_id: String,
    },
    CancelRequest {
        trade_id: String,
    },
    RegisterHub {},
    NewArbitrator {
        arbitrator: Addr,
        fiat: FiatCurrency,
    },
    DeleteArbitrator {
        arbitrator: Addr,
        fiat: FiatCurrency,
    },
    SettleDispute {
        trade_id: String,
        winner: Addr,
    },
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
pub enum TraderRole {
    Seller,
    Buyer,
    Arbitrator,
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
    pub maker_contact: Option<String>,
    pub arbitrator: Option<Addr>,
    pub offer_contract: Addr,
    pub offer_id: String,
    pub created_at: u64,
    pub denom: Denom,
    pub amount: Uint128,
    pub state: TradeState,
    pub fiat: FiatCurrency,
    pub state_history: Vec<TradeStateItem>,
}

impl Trade {
    pub fn get_state(&self) -> TradeState {
        return self.state.clone();
    }

    pub fn set_state(&mut self, new_state: TradeState, env: &Env, info: &MessageInfo) {
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
            .prefix(buyer)
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
            .prefix(seller)
            .range(storage, range_from, None, Order::Descending)
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, trade)| Ok(trade)))
            .collect();

        Ok(result)
    }

    pub fn trades_by_arbitrator(
        storage: &dyn Storage,
        arbitrator: String,
        last_value: Option<String>,
        limit: u32,
    ) -> StdResult<Vec<Trade>> {
        let range_from = match last_value {
            Some(thing) => Some(Bound::exclusive(thing)),
            None => None,
        };

        let result = trades()
            .idx
            .arbitrator
            .prefix(arbitrator)
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
    pub arbitrator: MultiIndex<'a, String, Trade, String>,
}

impl<'a> IndexList<Trade> for TradeIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Trade>> + '_> {
        let v: Vec<&dyn Index<Trade>> = vec![&self.buyer, &self.seller, &self.arbitrator];
        Box::new(v.into_iter())
    }
}

pub fn trades<'a>() -> IndexedMap<'a, String, Trade, TradeIndexes<'a>> {
    let indexes = TradeIndexes {
        buyer: MultiIndex::new(|t| t.buyer.to_string(), "trades", "trades__buyer"),
        seller: MultiIndex::new(|t| t.seller.to_string(), "trades", "trades__seller"),
        arbitrator: MultiIndex::new(
            |t| {
                t.arbitrator
                    .clone()
                    .unwrap_or(Addr::unchecked(""))
                    .to_string()
            },
            "trades",
            "trades__arbitrator",
        ),
    };
    IndexedMap::new("trades", indexes)
}

pub fn arbitrators<'a>() -> IndexedMap<'a, &'a str, Arbitrator, ArbitratorIndexes<'a>> {
    let indexes = ArbitratorIndexes {
        arbitrator: MultiIndex::new(
            |d: &Arbitrator| d.arbitrator.clone(),
            "arbitrators",
            "arbitrators__arbitrator",
        ),
        fiat: MultiIndex::new(
            |d: &Arbitrator| d.fiat.clone().to_string(),
            "arbitrators",
            "arbitrators__asset",
        ),
    };
    IndexedMap::new("arbitrators", indexes)
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
