use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::constants::{ACCEPTED_SUBSCRIPTIONS_KEY, PENDING_SUBSCRIPTIONS_KEY, STATE_KEY};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub commitment_denom: String,
    pub subscription_code_id: u64,
    pub recovery_admin: Addr,
    pub gp: Addr,
}

impl State {
    pub fn new(
        contract_addr: &Addr,
        subscription_code_id: u64,
        recovery_admin: &Addr,
        gp: &Addr,
    ) -> Self {
        Self {
            commitment_denom: format!("{}.commitment", contract_addr),
            subscription_code_id,
            recovery_admin: recovery_admin.clone(),
            gp: gp.clone(),
        }
    }
}

pub const STATE: Item<State> = Item::new(STATE_KEY);
pub const PENDING: Map<Addr, bool> = Map::new(PENDING_SUBSCRIPTIONS_KEY);
pub const ACCEPTED: Map<Addr, bool> = Map::new(ACCEPTED_SUBSCRIPTIONS_KEY);
