use cosmwasm_std::{Addr, Storage, Uint64};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::{constants::STATE_KEY, error::ContractError};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub gp: Addr,
    pub capital_denom: String,
    pub settlement_time: Option<Uint64>,
}

impl State {
    pub fn new(gp: Addr, capital_denom: String, settlement_time: Option<Uint64>) -> Self {
        Self {
            gp,
            capital_denom,
            settlement_time,
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
    Ok(state.settlement_time)
}

pub fn set_settlement_time(
    storage: &mut dyn Storage,
    new_settlement_time: Option<Uint64>,
) -> Result<(), ContractError> {
    let mut state = get(storage)?;
    state.settlement_time = new_settlement_time;
    set(storage, &state)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint64};
    use provwasm_mocks::mock_dependencies;

    use crate::storage::state::{get_settlement_time, set, State};

    use super::get;

    #[test]
    fn test_new_state() {
        let expected_addr = Addr::unchecked("address");
        let expected_capital_denom = "nhash";
        let expected_time = Some(Uint64::zero());
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            expected_time.clone(),
        );

        assert_eq!(expected_addr, state.gp);
        assert_eq!(expected_capital_denom, state.capital_denom);
        assert_eq!(expected_time, state.settlement_time);
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
        let expected_time = Some(Uint64::zero());
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            expected_time.clone(),
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
        let expected_time = None;
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            expected_time.clone(),
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
        let expected_time = Some(Uint64::zero());
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            expected_time.clone(),
        );
        set(deps.as_mut().storage, &state).unwrap();

        let settlement_time = get_settlement_time(&deps.storage).unwrap();
        assert_eq!(expected_time, settlement_time);
    }

    #[test]
    fn test_set_settlement() {
        let mut deps = mock_dependencies(&[]);
        let expected_addr = Addr::unchecked("address");
        let expected_capital_denom = "nhash";
        let expected_time = Some(Uint64::new(9999));
        let state = State::new(
            expected_addr.clone(),
            expected_capital_denom.to_string(),
            None,
        );

        set(deps.as_mut().storage, &state).unwrap();
        super::set_settlement_time(deps.as_mut().storage, expected_time.clone()).unwrap();
        let obtained = get(&deps.storage).unwrap();
        assert_eq!(expected_time, obtained.settlement_time);
    }
}
