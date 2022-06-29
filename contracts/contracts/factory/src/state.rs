use cw_storage_plus::Item;
use localterra_protocol::factory::{Admin, Config};

pub const CONFIG: Item<Config> = Item::new("config");
pub const ADMIN: Item<Admin> = Item::new("admin");
