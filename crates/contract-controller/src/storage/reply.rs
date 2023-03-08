use cosmwasm_std::{Addr, Order, Storage};
use cw_storage_plus::Map;

use crate::core::{constants::REPLIES_KEY, error::ContractError};

// We store our securities that we configured on initialization
pub const REPLIES_MAP: Map<u64, Addr> = Map::new(REPLIES_KEY);

pub fn add(storage: &mut dyn Storage, contract: &Addr) -> Result<u64, ContractError> {
    let index = get_next_index(storage)?;
    REPLIES_MAP.save(storage, index, contract)?;
    Ok(index)
}

pub fn remove(storage: &mut dyn Storage, index: u64) -> Result<Addr, ContractError> {
    let addr = REPLIES_MAP.load(storage, index)?;
    REPLIES_MAP.remove(storage, index);
    Ok(addr)
}

fn get_next_index(storage: &dyn Storage) -> Result<u64, ContractError> {
    let index = REPLIES_MAP
        .keys(storage, None, None, Order::Descending)
        .next();
    match index {
        None => Ok(0),
        Some(item) => Ok(item? + 1),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr};
    use provwasm_mocks::mock_dependencies;

    use crate::storage::reply::REPLIES_MAP;

    use super::{add, get_next_index, remove};

    #[test]
    fn test_add() {
        let mut deps = mock_dependencies(&[]);
        let contract1 = Addr::unchecked("address1");
        let contract2 = Addr::unchecked("address2");

        let id = add(deps.as_mut().storage, &contract1).unwrap();
        assert_eq!(0, id);
        let stored = REPLIES_MAP.load(&deps.storage, id).unwrap();
        assert_eq!(stored, contract1);

        let id = add(deps.as_mut().storage, &contract2).unwrap();
        assert_eq!(1, id);
        let stored = REPLIES_MAP.load(&deps.storage, id).unwrap();
        assert_eq!(stored, contract2);
    }

    #[test]
    fn test_remove_success() {
        let mut deps = mock_dependencies(&[]);
        let contract1 = Addr::unchecked("address1");
        let contract2 = Addr::unchecked("address2");

        let id1 = add(deps.as_mut().storage, &contract1).unwrap();
        let id2 = add(deps.as_mut().storage, &contract2).unwrap();

        let value = remove(deps.as_mut().storage, id1).unwrap();
        assert_eq!(contract1, value);
        assert_eq!(false, REPLIES_MAP.has(&deps.storage, id1));
        assert_eq!(true, REPLIES_MAP.has(&deps.storage, id2));
    }

    #[test]
    fn test_remove_non_existant() {
        let mut deps = mock_dependencies(&[]);
        let contract1 = Addr::unchecked("address1");
        let contract2 = Addr::unchecked("address2");

        add(deps.as_mut().storage, &contract1).unwrap();
        add(deps.as_mut().storage, &contract2).unwrap();
        let id = 3;

        remove(deps.as_mut().storage, id).unwrap_err();
    }

    #[test]
    fn test_get_next_index_first() {
        let deps = mock_dependencies(&[]);
        assert_eq!(0, get_next_index(&deps.storage).unwrap());
    }

    #[test]
    fn test_get_next_index_multiple() {
        let mut deps = mock_dependencies(&[]);
        let contract1 = Addr::unchecked("address1");

        add(deps.as_mut().storage, &contract1).unwrap();
        assert_eq!(1, get_next_index(&deps.storage).unwrap());
    }
}
