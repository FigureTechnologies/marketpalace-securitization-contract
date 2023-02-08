use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::execute::settlement::commitment::Commitment;

use super::{
    constants::{
        AVAILABLE_CAPITAL_KEY, COMMITS_KEY, PAID_IN_CAPITAL_KEY, SECURITIES_MAP_KEY, STATE_KEY,
    },
    rules::InvestmentVehicleRule,
    security::{Security, SecurityCommitment},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub gp: Addr,
    pub capital_denom: String,
    pub rules: Vec<InvestmentVehicleRule>,
}

impl State {
    pub fn new(gp: Addr, capital_denom: String, rules: Vec<InvestmentVehicleRule>) -> Self {
        Self {
            gp,
            capital_denom,
            rules,
        }
    }
}

// We store basic contract State
pub const STATE: Item<State> = Item::new(STATE_KEY);

// We store our securities that we configured on initialization
pub const SECURITIES_MAP: Map<String, Security> = Map::new(SECURITIES_MAP_KEY);

// All the propose, accepted, and settled commitments
pub const COMMITS: Map<Addr, Commitment> = Map::new(COMMITS_KEY);
pub const PAID_IN_CAPITAL: Map<Addr, Vec<SecurityCommitment>> = Map::new(PAID_IN_CAPITAL_KEY);
pub const AVAILABLE_CAPITAL: Map<Addr, Vec<Coin>> = Map::new(AVAILABLE_CAPITAL_KEY);

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;

    use crate::core::{
        rules::{InvestmentVehicleRule, SettlementDate},
        state::State,
    };

    #[test]
    fn test_new_state() {
        let expected_addr = Addr::unchecked("address");
        let expected_capital_denom = "nhash";
        let expected_rules = vec![InvestmentVehicleRule::SettlementDate {
            0: SettlementDate {},
        }];
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            expected_rules.clone(),
        );

        assert_eq!(expected_addr, state.gp);
        assert_eq!(expected_capital_denom, state.capital_denom);
        assert_eq!(expected_rules, state.rules);
    }
}
