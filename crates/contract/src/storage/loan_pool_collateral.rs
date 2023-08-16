use crate::core::collateral::LoanPoolMarkerCollateral;
use crate::core::constants::LOAN_POOL_COLLATERAL;
use crate::core::error::ContractError;
use cosmwasm_std::{Addr, Order, Storage};
use cw_storage_plus::Map;

pub const COLLATERAL: Map<Addr, LoanPoolMarkerCollateral> = Map::new(LOAN_POOL_COLLATERAL);

pub fn get(
    storage: &dyn Storage,
    marker_address: Addr,
) -> Result<LoanPoolMarkerCollateral, ContractError> {
    Ok(COLLATERAL.load(storage, marker_address)?)
}

pub fn set(
    storage: &mut dyn Storage,
    collateral: &LoanPoolMarkerCollateral,
) -> Result<(), ContractError> {
    Ok(COLLATERAL.save(storage, collateral.marker_address.clone(), collateral)?)
}

pub fn remove(
    storage: &mut dyn Storage,
    collateral: &LoanPoolMarkerCollateral,
) -> Result<(), ContractError> {
    COLLATERAL.remove(storage, collateral.marker_address.clone());
    Ok(())
}

pub fn exists(storage: &dyn Storage, lp: Addr) -> bool {
    COLLATERAL.has(storage, lp)
}

pub fn get_with_state(
    storage: &dyn Storage,
    state: LoanPoolMarkerCollateral,
) -> Vec<LoanPoolMarkerCollateral> {
    let collateral: Vec<LoanPoolMarkerCollateral> = COLLATERAL
        .range(storage, None, None, Order::Ascending)
        .filter(|item| item.is_ok() && item.as_ref().unwrap().1.marker_denom == state.marker_denom)
        .map(|item| item.unwrap().1)
        .collect();
    collateral
}

pub fn get_all_states(storage: &dyn Storage) -> Vec<LoanPoolMarkerCollateral> {
    let collateral: Vec<LoanPoolMarkerCollateral> = COLLATERAL
        .range(storage, None, None, Order::Ascending)
        .map(|item| item.unwrap().1)
        .collect();
    collateral
}


#[cfg(test)]
mod tests {
    use super::*;
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_get_and_set() {
        let mut deps = mock_dependencies(&[]);
        let marker_address = Addr::unchecked("addr1");
        let collateral = LoanPoolMarkerCollateral::new(
            marker_address.clone(),
            "denom".to_string(),
            100,
            Vec::new(),
        );

        // Test setting collateral
        set(&mut deps.storage, &collateral).unwrap();
        let result = get(&deps.storage, marker_address.clone()).unwrap();
        assert_eq!(result, collateral);

        // Test removing collateral
        remove(&mut deps.storage, &result.clone()).unwrap();
        let result = get(&deps.storage, marker_address.clone());
        assert!(result.is_err()); // Expect an error because the collateral has been removed
    }

    #[test]
    fn test_exists() {
        let mut deps = mock_dependencies(&[]);
        let marker_address = Addr::unchecked("addr1");
        let collateral = LoanPoolMarkerCollateral::new(
            marker_address.clone(),
            "denom".to_string(),
            100,
            Vec::new(),
        );

        // Test existence after setting
        set(&mut deps.storage, &collateral).unwrap();
        assert!(exists(&deps.storage, marker_address.clone()));

        // Test existence after removing
        remove(&mut deps.storage, &collateral.clone()).unwrap();
        assert!(!exists(&deps.storage, marker_address.clone()));
    }

    // Add more tests as needed for other functions and edge cases
}
