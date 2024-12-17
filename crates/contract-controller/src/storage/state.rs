use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::{constants::STATE_KEY, error::ContractError};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub batch_size: u128,
    pub migrating: bool,
    pub last_address: Option<Addr>,
}

impl State {
    pub fn new(batch_size: u128) -> Self {
        Self {
            batch_size,
            migrating: false,
            last_address: None,
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

pub fn update_batch_size(
    storage: &mut dyn Storage,
    batch_size: u128,
) -> Result<State, ContractError> {
    STATE.update(storage, |mut state| -> Result<State, ContractError> {
        state.batch_size = batch_size;
        Ok(state)
    })
}

pub fn is_migrating(storage: &dyn Storage) -> Result<bool, ContractError> {
    let state = get(storage)?;
    Ok(state.migrating)
}

#[cfg(test)]
mod tests {

    use provwasm_mocks::mock_provenance_dependencies;

    use crate::storage::state::{is_migrating, set, update_batch_size};

    use super::{get, State};

    #[test]
    fn test_new_state_creation() {
        let state = State::new(2);
        assert_eq!(2, state.batch_size);
        assert_eq!(false, state.migrating);
        assert_eq!(None, state.last_address);
    }

    #[test]
    fn test_get_set() {
        let mut deps = mock_provenance_dependencies();
        let state = State::new(2);

        set(deps.as_mut().storage, &state).unwrap();
        let new_state = get(&deps.storage).unwrap();

        assert_eq!(2, new_state.batch_size);
        assert_eq!(false, new_state.migrating);
        assert_eq!(None, new_state.last_address);
    }

    #[test]
    fn test_update_batch_size() {
        let mut deps = mock_provenance_dependencies();
        let state = State::new(2);

        set(deps.as_mut().storage, &state).unwrap();
        let returned_state = update_batch_size(deps.as_mut().storage, 5).unwrap();

        let new_state = get(&deps.storage).unwrap();
        assert_eq!(5, new_state.batch_size);
        assert_eq!(false, new_state.migrating);
        assert_eq!(None, new_state.last_address);
        assert_eq!(returned_state, new_state);
    }

    #[test]
    fn test_is_migrating() {
        let mut deps = mock_provenance_dependencies();
        let mut state = State::new(2);

        set(deps.as_mut().storage, &state).unwrap();
        assert_eq!(false, is_migrating(&deps.storage).unwrap());

        state.migrating = true;
        set(deps.as_mut().storage, &state).unwrap();
        assert_eq!(true, is_migrating(&deps.storage).unwrap());
    }
}
