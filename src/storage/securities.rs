use cw_storage_plus::Map;

use crate::core::{constants::SECURITIES_MAP_KEY, security::Security};

// We store our securities that we configured on initialization
pub const SECURITIES_MAP: Map<String, Security> = Map::new(SECURITIES_MAP_KEY);
