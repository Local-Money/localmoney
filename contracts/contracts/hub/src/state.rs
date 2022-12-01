use cw_storage_plus::Item;

use localmoney_protocol::hub::{Admin, HubConfig};

pub const CONFIG: Item<HubConfig> = Item::new("config");
pub const ADMIN: Item<Admin> = Item::new("admin");
