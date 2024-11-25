use crate::core::collateral::LoanPoolMarkerCollateral;
use crate::core::constants::LOAN_POOL_COLLATERAL;
use crate::core::error::ContractError;
use cosmwasm_std::{Addr, Order, Storage};
use cw_storage_plus::Map;

// Constant for the COLLATERAL storage map with an Addr as key and LoanPoolMarkerCollateral as value
pub const COLLATERAL: Map<Addr, LoanPoolMarkerCollateral> = Map::new(LOAN_POOL_COLLATERAL);

/// Get Collateral from storage.
///
/// ## Arguments
///
/// - `storage`: A reference to the contract's storage
/// - `marker_address`: The address of a marker within the storage
///
/// ## Returns
///
/// - Returns a `Result` with `Ok` variant containing the `LoanPoolMarkerCollateral` if the get
/// operation is successful, otherwise a `Err` variant containing `ContractError`.
pub fn get(
    storage: &dyn Storage,
    marker_address: Addr,
) -> Result<LoanPoolMarkerCollateral, ContractError> {
    Ok(COLLATERAL.load(storage, marker_address)?)
}

/// Save Collateral to storage.
///
/// - `storage`: A mutable reference to the contract's storage
/// - `collateral`: The `LoanPoolMarkerCollateral` data to be saved to storage
///
/// ## Returns
///
/// - Returns a `Result` with `Ok` variant if the save operation is successful,
/// otherwise a `Err` variant containing `ContractError`.
pub fn set(
    storage: &mut dyn Storage,
    collateral: &LoanPoolMarkerCollateral,
) -> Result<(), ContractError> {
    Ok(COLLATERAL.save(storage, collateral.marker_address.clone(), collateral)?)
}

/// This function is declared as a public function named `remove`. This means it can be called from other modules.
/// It expects two parameters:
/// `storage`: a mutable reference to a trait object of dyn Storage. Traits are a way to group method signatures
///           that define a specific behavior in a generic way. `dyn` keyword is used for dynamic dispatch.
/// `collateral`: an immutable reference to an instance of LoanPoolMarkerCollateral.
pub fn remove(
    storage: &mut dyn Storage,
    collateral: &LoanPoolMarkerCollateral,
) -> Result<(), ContractError> {
    COLLATERAL.remove(storage, collateral.marker_address.clone());
    Ok(())
}

/// function defining if the marker has been added to the pool.
/// # Parameters
///
/// * `storage`: It takes a shared reference to a dynamic Storage trait object.
/// * `marker_address`: An instance of `Addr` which is likely used to represent a specific address or location within the storage.
///
/// This function checks if a certain marker address exists within the storage by calling the `has` method on the `COLLATERAL` variable.
///
/// # Returns
///
/// A boolean value. Returning `true` if the `marker_address` exists in the `COLLATERAL`, otherwise `false`.
///
/// # Note
///
/// It's important to remember the behavior of the function could differ considerably based on the specific implementation of the `COLLATERAL`, `Storage` trait and `has` method.
pub fn exists(storage: &dyn Storage, marker_address: Addr) -> bool {
    COLLATERAL.has(storage, marker_address)
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
    use provwasm_std::types::provenance::marker::v1::{Access, AccessGrant};

    #[test]
    fn test_get_and_set() {
        let mut deps = mock_dependencies(&[]);
        let marker_address = Addr::unchecked("addr1");

        let collateral = sample_collateral("addr1", "denom", 100, Vec::new(), "owner");
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
    fn test_get_and_set_non_empty_permissions() {
        let mut deps = mock_dependencies(&[]);
        let marker_address = Addr::unchecked("addr1");
        let permissions = vec![AccessGrant {
            permissions: vec![Access::Mint as i32, Access::Transfer as i32],
            address: "addr2".to_string(),
        }];
        let collateral = sample_collateral("addr1", "denom", 100, permissions.clone(), "owner");

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
        let collateral = sample_collateral("addr1", "denom", 100, Vec::new(), "owner");

        // Test existence after setting
        set(&mut deps.storage, &collateral).unwrap();
        assert!(exists(&deps.storage, collateral.marker_address.clone()));

        // Test existence after removing
        remove(&mut deps.storage, &collateral.clone()).unwrap();
        assert!(!exists(&deps.storage, collateral.marker_address.clone()));
    }

    fn sample_collateral(
        addr: &str,
        denom: &str,
        amount: u128,
        permissions: Vec<AccessGrant>,
        original_owner: &str,
    ) -> LoanPoolMarkerCollateral {
        let marker_address = Addr::unchecked(addr);
        let original_owner = Addr::unchecked(original_owner);

        LoanPoolMarkerCollateral::new(
            marker_address.clone(),
            denom.to_string(),
            amount,
            original_owner,
            permissions,
        )
    }

    #[test]
    fn test_get_with_state() {
        let mut deps = mock_dependencies(&[]);

        // Set up different collaterals
        let collateral1 = sample_collateral("addr1", "denom1", 100, Vec::new(), "owner");
        let collateral2 = sample_collateral("addr2", "denom1", 200, Vec::new(), "owner");
        let collateral3 = sample_collateral("addr3", "denom2", 300, Vec::new(), "owner");

        // Store them
        set(&mut deps.storage, &collateral1).unwrap();
        set(&mut deps.storage, &collateral2).unwrap();
        set(&mut deps.storage, &collateral3).unwrap();

        // Search with denom1 state
        let results = get_with_state(
            &deps.storage,
            sample_collateral("", "denom1", 0, Vec::new(), "owner"),
        );
        assert_eq!(results.len(), 2);
        assert_eq!(results.contains(&collateral1), true);
        assert_eq!(results.contains(&collateral2), true);
    }

    #[test]
    fn test_get_all_states() {
        let mut deps = mock_dependencies(&[]);

        // Set up different collaterals
        let collateral1 = sample_collateral("addr1", "denom1", 100, Vec::new(), "owner");
        let collateral2 = sample_collateral("addr2", "denom1", 200, Vec::new(), "owner");
        let collateral3 = sample_collateral("addr3", "denom2", 300, Vec::new(), "owner");

        // Store them
        set(&mut deps.storage, &collateral1).unwrap();
        set(&mut deps.storage, &collateral2).unwrap();
        set(&mut deps.storage, &collateral3).unwrap();

        let collaterals = get_all_states(&deps.storage);
        assert_eq!(collaterals.len(), 3);
        assert_eq!(collaterals.contains(&collateral1), true);
        assert_eq!(collaterals.contains(&collateral2), true);
        assert_eq!(collaterals.contains(&collateral3), true);
    }
}
