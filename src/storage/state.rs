use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::{constants::STATE_KEY, error::ContractError, rules::InvestmentVehicleRule};

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

pub fn get(storage: &dyn Storage) -> Result<State, ContractError> {
    Ok(STATE.load(storage)?)
}

pub fn set(storage: &mut dyn Storage, state: &State) -> Result<(), ContractError> {
    Ok(STATE.save(storage, state)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::rules::{InvestmentVehicleRule, SettlementDate},
        storage::state::{set, State},
    };

    use super::get;

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

    #[test]
    fn test_get_invalid() {
        let deps = mock_dependencies(&[]);
        get(&deps.storage).unwrap_err();
    }

    #[test]
    fn test_get_set_valid() {
        let mut deps = mock_dependencies(&[]);
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

        set(deps.as_mut().storage, &state).unwrap();

        let obtained = get(&deps.storage).unwrap();
        assert_eq!(state, obtained);
    }
}
