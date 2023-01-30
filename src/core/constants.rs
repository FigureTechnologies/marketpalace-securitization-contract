// Version
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const STATE_KEY: &str = "state";
pub const PENDING_COMMITS_KEY: &str = "pending";
pub const ACCEPTED_COMMITS_KEY: &str = "accepted";
pub const COMMITS_KEY: &str = "commits";
pub const SECURITIES_MAP_KEY: &str = "security_types_map";
pub const SECURITIES_LIST_KEY: &str = "security_types_list";
