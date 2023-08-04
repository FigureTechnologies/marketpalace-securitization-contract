// Version
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const STATE_KEY: &str = "state";
pub const COMMITS_KEY: &str = "commits";
pub const PAID_IN_CAPITAL_KEY: &str = "paid_in_capital";
pub const SECURITIES_MAP_KEY: &str = "security_types_map";
pub const REMAINING_SECURITIES_KEY: &str = "remaining_securities_map";
pub const AVAILABLE_CAPITAL_KEY: &str = "available_capital";
pub const LOAN_POOL_COLLATERAL: &str = "paid_in_capital";
pub const WHITELIST_CONTRIBUTORS: &str = "whitelist_contributors";

