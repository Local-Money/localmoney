use crate::contract::CONTRACT_NAME;
use cw_storage_plus::Map;
use localterra_protocol::profile::Profile;

pub const PROFILE: Map<String, Profile> = Map::new(CONTRACT_NAME);
