use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::subscription::Subscription;

use super::{
    constants::{
        ACCEPTED_SUBSCRIPTIONS_KEY, PENDING_SUBSCRIPTIONS_KEY, SECURITY_TYPES_KEY, STATE_KEY,
    },
    msg::Security,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub gp: Addr,
}

impl State {
    pub fn new(gp: Addr) -> Self {
        Self { gp }
    }
}

pub const STATE: Item<State> = Item::new(STATE_KEY);
pub const PENDING: Map<Addr, Subscription> = Map::new(PENDING_SUBSCRIPTIONS_KEY);
pub const ACCEPTED: Map<Addr, Subscription> = Map::new(ACCEPTED_SUBSCRIPTIONS_KEY);
pub const SECURITY_TYPES: Map<String, Security> = Map::new(SECURITY_TYPES_KEY);
