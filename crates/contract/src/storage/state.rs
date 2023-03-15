use cosmwasm_std::{Addr, Storage, Uint64};
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

pub fn get_settlement_time(storage: &dyn Storage) -> Result<Option<Uint64>, ContractError> {
    let state = get(storage)?;

    let duration: Option<Uint64> = state
        .rules
        .iter()
        .map(|rule| match rule {
            InvestmentVehicleRule::SettlementTime(offset) => offset.to_owned(),
        })
        .next();

    Ok(duration)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint64};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::rules::InvestmentVehicleRule,
        storage::state::{get_settlement_time, set, State},
    };

    use super::get;

    #[test]
    fn test_new_state() {
        let expected_addr = Addr::unchecked("address");
        let expected_capital_denom = "nhash";
        let expected_rules = vec![InvestmentVehicleRule::SettlementTime { 0: Uint64::zero() }];
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
        let expected_rules = vec![InvestmentVehicleRule::SettlementTime { 0: Uint64::zero() }];
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            expected_rules.clone(),
        );

        set(deps.as_mut().storage, &state).unwrap();

        let obtained = get(&deps.storage).unwrap();
        assert_eq!(state, obtained);
    }

    #[test]
    fn test_get_settlement_time_not_set() {
        let mut deps = mock_dependencies(&[]);
        let expected_addr = Addr::unchecked("address");
        let expected_capital_denom = "nhash";
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            vec![],
        );
        set(deps.as_mut().storage, &state).unwrap();

        let settlement_time = get_settlement_time(&deps.storage).unwrap();
        assert_eq!(None, settlement_time);
    }

    #[test]
    fn test_get_settlement_time_set() {
        let mut deps = mock_dependencies(&[]);
        let expected_addr = Addr::unchecked("address");
        let expected_capital_denom = "nhash";
        let expected_rules = vec![InvestmentVehicleRule::SettlementTime { 0: Uint64::zero() }];
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            expected_rules.clone(),
        );
        set(deps.as_mut().storage, &state).unwrap();

        let settlement_time = get_settlement_time(&deps.storage).unwrap();
        assert_eq!(Some(Uint64::zero()), settlement_time);
    }
}
