use cosmwasm_std::{Addr, Order, Storage};
use cw_storage_plus::{Bound, Map};

use crate::core::{constants::CONTRACT_KEY, error::ContractError};

// We store our securities that we configured on initialization
pub const CONTRACTS_MAP: Map<&Addr, bool> = Map::new(CONTRACT_KEY);

pub fn has(storage: &dyn Storage, contract: &Addr) -> bool {
    CONTRACTS_MAP.has(storage, contract)
}

pub fn add(storage: &mut dyn Storage, contract: &Addr) -> Result<(), ContractError> {
    Ok(CONTRACTS_MAP.save(storage, contract, &true)?)
}

pub fn remove(storage: &mut dyn Storage, contract: &Addr) {
    CONTRACTS_MAP.remove(storage, contract);
}

pub fn list(storage: &dyn Storage) -> Vec<Addr> {
    let contracts: Vec<Addr> = CONTRACTS_MAP
        .keys(storage, None, None, Order::Ascending)
        .map(|item| item.unwrap())
        .collect();
    contracts
}

pub fn range(storage: &dyn Storage, start: Option<&Addr>, amount: u128) -> Vec<Addr> {
    let min = start.map(Bound::exclusive);
    let contracts: Vec<Addr> = match amount {
        0 => CONTRACTS_MAP
            .keys(storage, min, None, Order::Ascending)
            .map(|item| item.unwrap())
            .collect(),
        _ => CONTRACTS_MAP
            .keys(storage, min, None, Order::Ascending)
            .take(amount as usize)
            .map(|item| item.unwrap())
            .collect(),
    };
    contracts
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use provwasm_mocks::mock_dependencies;

    use crate::storage::contract::{add, range, remove};

    use super::{has, list, CONTRACTS_MAP};

    #[test]
    fn test_has_success() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        CONTRACTS_MAP
            .save(deps.as_mut().storage, &contract, &true)
            .unwrap();
        assert_eq!(true, has(&deps.storage, &contract));
    }

    #[test]
    fn test_has_failure() {
        let deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        assert_eq!(false, has(&deps.storage, &contract));
    }

    #[test]
    fn test_add() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &contract).unwrap();
        add(deps.as_mut().storage, &contract2).unwrap();
        assert_eq!(true, has(&deps.storage, &contract));
        assert_eq!(true, has(&deps.storage, &contract2));
    }

    #[test]
    fn test_remove() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &contract).unwrap();
        add(deps.as_mut().storage, &contract2).unwrap();
        remove(deps.as_mut().storage, &contract);
        assert_eq!(false, has(&deps.storage, &contract));
        assert_eq!(true, has(&deps.storage, &contract2));
    }

    #[test]
    fn test_remove_non_existant() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        remove(deps.as_mut().storage, &contract);
        assert!(true);
    }

    #[test]
    fn test_list_non_empty() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &contract).unwrap();
        add(deps.as_mut().storage, &contract2).unwrap();
        let addresses = list(&deps.storage);

        assert_eq!(vec![contract, contract2], addresses);
    }

    #[test]
    fn test_list_empty() {
        let deps = mock_dependencies(&[]);
        let addresses = list(&deps.storage);
        let expected: Vec<Addr> = vec![];
        assert_eq!(expected, addresses);
    }

    #[test]
    fn test_range_starts_at_beginning() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &contract).unwrap();
        add(deps.as_mut().storage, &contract2).unwrap();
        let addresses = range(&deps.storage, None, 1);

        assert_eq!(vec![contract], addresses);
    }

    #[test]
    fn test_range_starts_at_location() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &contract).unwrap();
        add(deps.as_mut().storage, &contract2).unwrap();
        let addresses = range(&deps.storage, None, 1);
        let addresses = range(&deps.storage, Some(&addresses[0]), 1);

        assert_eq!(vec![contract2], addresses);
    }

    #[test]
    fn test_range_handles_zero_elements() {
        let deps = mock_dependencies(&[]);
        let addresses = range(&deps.storage, None, 1);
        let expected: Vec<Addr> = vec![];

        assert_eq!(expected, addresses);
    }

    #[test]
    fn test_range_returns_all_for_zero_length() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &contract).unwrap();
        add(deps.as_mut().storage, &contract2).unwrap();
        let addresses = range(&deps.storage, None, 0);

        assert_eq!(vec![contract, contract2], addresses);
    }

    #[test]
    fn test_range_doesnt_exceed_elements() {
        let mut deps = mock_dependencies(&[]);
        let contract = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &contract).unwrap();
        add(deps.as_mut().storage, &contract2).unwrap();
        let addresses = range(&deps.storage, None, 5);

        assert_eq!(vec![contract, contract2], addresses);
    }
}
