use cw_storage_plus::Item;
use localterra_protocol::factory::Config;

pub const CONFIG: Item<Config> = Item::new("config");
