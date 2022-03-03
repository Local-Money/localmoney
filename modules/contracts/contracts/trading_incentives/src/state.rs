use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};
use localterra_protocol::trading_incentives::Config;

pub const CONFIG: Item<Config> = Item::new("config");
pub const TOTAL_VOLUME: Map<&[u8], Uint128> = Map::new("total_volume");
pub const TRADER_VOLUME: Map<(&[u8], &[u8]), Uint128> = Map::new("trader_volume");
