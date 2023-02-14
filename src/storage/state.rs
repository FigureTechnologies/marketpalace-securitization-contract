use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::{constants::STATE_KEY, rules::InvestmentVehicleRule};

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

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;

    use crate::{
        core::rules::{InvestmentVehicleRule, SettlementDate},
        storage::state::State,
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
