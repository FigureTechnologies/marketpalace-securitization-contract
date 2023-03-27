// Version
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const STATE_KEY: &str = "state";
pub const CONTRACT_KEY: &str = "contract";
pub const REPLIES_KEY: &str = "replies";

pub const REPLY_INIT_ID: u64 = 0;
pub const REPLY_STARTING_ID: u64 = 1;
