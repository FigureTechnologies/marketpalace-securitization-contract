use crate::core::constants::WHITELIST_CONTRIBUTORS;
use crate::core::error::ContractError;
use crate::core::security::LoanPoolContributors;
use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;

pub const WHITELIST: Item<Vec<Addr>> = Item::new(WHITELIST_CONTRIBUTORS);

impl LoanPoolContributors {
    pub fn human_whitelist(&self) -> Vec<String> {
        self.addresses.iter().map(|a| a.to_string()).collect()
    }
}

/// Adds a list of new contributors to the existing whitelist.
///
/// # Arguments
///
/// * `storage` - A mutable reference to the contract's storage.
/// * `new_contributors` - A vector of addresses (contributors) to be added to the whitelist.
///
/// # Returns
///
/// * A `Result` which is:
///     - `Ok(())` on success.
///     - `Err(ContractError)` on failure, where `ContractError` is an enum defined within the contract to handle possible error cases.
///
/// # Errors
///
/// Will return `Err(ContractError::Std(StdError))` where `StdError` is the error returned from the storage API
/// if it fails to load or save data from/to storage.
///
/// # Example
///
/// ```ignore
/// let new_contributors = vec![Addr::unchecked("addr1"), Addr::unchecked("addr2")];
/// save_contributors(deps.storage, new_contributors);
/// ```
pub fn save_contributors(
    storage: &mut dyn Storage,
    new_contributors: Vec<Addr>,
) -> Result<(), ContractError> {
    // Load current contributors from the storage
    let mut contributors = WHITELIST.load(storage).unwrap_or_else(|_| vec![]);
    // Extend the updated list with the new contributors
    contributors.extend(new_contributors);
    // Save updated contributors back to the storage
    WHITELIST.save(storage, &contributors)?;

    Ok(())
}

/// Removes a list of contributors from the whitelist.
///
/// # Arguments
///
/// * `storage` - A mutable reference to the contract's storage.
/// * `remove_contributors` - A vector of addresses (contributors) to be removed from the whitelist.
///
/// # Returns
///
/// * A `Result` which is:
///     - `Ok(())` on success.
///     - `Err(ContractError)` on failure, where `ContractError` is an enum defined within the contract to handle possible error cases.
///
/// # Errors
///
/// Will return `Err(ContractError::Std(StdError))` where `StdError` is the error returned from the storage API
/// if it fails to load or save data from/to storage.
///
/// # Example
///
/// ```ignore
/// let remove_contributors = vec![Addr::unchecked("addr1"), Addr::unchecked("addr2")];
/// remove_contributors(deps.storage, remove_contributors);
/// ```
pub fn remove_contributors(
    storage: &mut dyn Storage,
    remove_contributors: Vec<Addr>,
) -> Result<(), ContractError> {
    // Load current contributors from the storage
    let mut contributors = WHITELIST.load(storage).unwrap_or_else(|_| vec![]);

    // Retain only those contributors that are not in the removal list
    contributors.retain(|contributor| !remove_contributors.contains(contributor));

    // Save updated contributors back to the storage
    WHITELIST.save(storage, &contributors)?;

    Ok(())
}
/// This function retrieves the list of contributors from the whitelist in the given storage.
///
/// # Arguments
///
/// * `storage` - A storage object implementing the `Storage` trait. This is where the function
///   looks for the whitelist of contributors.
///
/// # Returns
///
/// * A `Vec<Addr>` representing the list of contributors. If the whitelist cannot be loaded from storage,
///   this function will return an empty `Vec<Addr>`.
///
/// # Examples
///
/// ```
/// use your_storage_lib::Storage;
/// use your_other_lib::Addr;
/// use your_utils_lib::Item;
///
/// // Assuming a valid storage object and a populated whitelist
/// let storage = get_storage(); // Placeholder function
/// let contributors = get_whitelist_contributors(&storage);
/// println!("{:#?}", contributors);
/// ```
///
/// # Panics
///
/// This function will panic if an error occurs while loading the whitelist from storage and will return
/// an empty vector as default.
///
/// # Safety
///
/// This function assumes the storage provided can correctly and safely load the items requested.
pub fn get_whitelist_contributors(storage: &dyn Storage) -> Vec<Addr> {
    WHITELIST.load(storage).unwrap_or_else(|_| vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::{Addr, StdResult};

    #[test]
    fn test_save_contributors() -> StdResult<()> {
        let mut storage = MockStorage::new();
        let addr1 = Addr::unchecked("addr1");
        let addr2 = Addr::unchecked("addr2");

        // Test saving some contributors
        let contributors = vec![addr1.clone(), addr2.clone()];
        save_contributors(&mut storage, contributors.clone()).unwrap();
        let stored_contributors: Vec<Addr> = WHITELIST.load(&storage).unwrap();
        assert_eq!(stored_contributors, contributors);

        // Test saving an additional contributor, which should append to the existing ones
        let addr3 = Addr::unchecked("addr3");
        let additional_contributors = vec![addr3.clone()];
        save_contributors(&mut storage, additional_contributors.clone()).unwrap();
        let expected_contributors = vec![addr1.clone(), addr2.clone(), addr3.clone()];
        let stored_contributors: Vec<Addr> = WHITELIST.load(&mut storage).unwrap();
        assert_eq!(stored_contributors, expected_contributors);

        Ok(())
    }

    #[test]
    fn test_remove_contributors_no_key() -> StdResult<()> {
        let mut storage = MockStorage::new();
        let contributors = vec![Addr::unchecked("addr1"), Addr::unchecked("addr2")];

        // Test removing contributors when the whitelist key doesn't exist
        // This should still succeed because remove_contributors() is designed to handle this gracefully
        let result = remove_contributors(&mut storage, contributors.clone());
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_remove_contributors() -> StdResult<()> {
        let contributors = vec![Addr::unchecked("addr1"), Addr::unchecked("addr2")];
        let mut storage = MockStorage::new();
        // Add contributors to the whitelist
        save_contributors(&mut storage, contributors.clone()).unwrap();

        // Test removing contributors
        remove_contributors(&mut storage, contributors.clone()).unwrap();

        // Assert that the contributors have been removed
        let whitelist = WHITELIST.load(&mut storage).unwrap();
        assert!(whitelist.is_empty());

        Ok(())
    }

    #[test]
    fn test_get_whitelist_contributors() -> StdResult<()> {
        let mut storage = MockStorage::new();
        let addr1 = Addr::unchecked("addr1");
        let addr2 = Addr::unchecked("addr2");

        // Test getting contributors when there are none
        let empty_contributors = get_whitelist_contributors(&storage);
        assert!(empty_contributors.is_empty());

        // Test getting contributors when there are some
        let contributors = vec![addr1.clone(), addr2.clone()];
        save_contributors(&mut storage, contributors.clone()).unwrap();
        let stored_contributors = get_whitelist_contributors(&storage);
        assert_eq!(stored_contributors, contributors);

        Ok(())
    }
}
