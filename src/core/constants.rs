// Version
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const STATE_KEY: &str = "state";
pub const PENDING_SUBSCRIPTIONS_KEY: &str = "pending";
pub const ACCEPTED_SUBSCRIPTIONS_KEY: &str = "accepted";
pub const SECURITY_TYPES_KEY: &str = "security_types";
