use cosmwasm_std::{Addr, Response};

use crate::storage::whitelist_contributors_store::remove_contributors;
use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::state::{self},
};

/// Handle function that processes removal of contributors from the loan pool whitelist.
///
/// This function gets triggered when a request to remove loan pool contributors is made.
/// It first verifies that only the "gp" is authorized to make this request.
/// It then calls the helper function `remove_loan_pool_contributors` to remove contributors from the loan pool.
///
/// Parameters:
/// * `deps`: the storage dependency object that provides access to the relevant dependencies.
/// * `sender`: the address of the entity attempting to remove a contributor.
/// * `contributors`: a vector of contributor addresses that are supposed to be removed from the whitelist.
///
/// Returns:
/// * `ProvTxResponse`: the status of the operation in the form of `ProvTxResponse` object.
///
/// # Panics
/// * if the sender is not the same as the "gp".
/// * if the contract's storage is not populated.
pub fn handle(mut deps: ProvDepsMut, sender: Addr, contributors: Vec<Addr>) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        // only gp can add whitelisted contributor
        return Err(ContractError::Unauthorized {});
    }

    remove_loan_pool_contributors(&mut deps, contributors)
}

/// Helper function to remove contributors from the loan pool.
///
/// This function is invoked by the handle function to execute the removal of contributors.
/// It removes the contributors from the contract storage and constructs a response
/// indicating success and specifying the contributors that were removed.
///
/// Parameters:
/// * `deps`: the storage dependency object that provides access to the relevant dependencies.
/// * `loan_pool_contributors`: a vector of contributor addresses that are supposed to be removed from the loan pool.
///
/// Returns:
/// * `ProvTxResponse`: the status of the operation in the form of `ProvTxResponse` object.
///
/// # Panics
/// * if unable to remove contributors from the contract's storage.
pub fn remove_loan_pool_contributors(
    deps: &mut ProvDepsMut,
    loan_pool_contributors: Vec<Addr>,
) -> ProvTxResponse {
    remove_contributors(deps.storage, loan_pool_contributors.clone())?;

    // Converting Vec<Addr> to Vec<String>
    let contributors_as_str: Vec<String> = loan_pool_contributors
        .into_iter()
        .map(|addr| addr.to_string())
        .collect();
    // Joining Vec<String> into a single String
    let contributors_str = contributors_as_str.join(",");

    let response = Response::new()
        .add_attribute("action", "whitelist_removed")
        .add_attribute("whitelist_address_removed", contributors_str);

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::testing::create_test_state;
    use cosmwasm_std::testing::mock_env;
    use cosmwasm_std::{Addr, StdResult};
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_remove_contributors() -> StdResult<()> {
        let gp = Addr::unchecked("gp");

        let mut deps = mock_dependencies(&[]);
        create_test_state(&mut deps, &mock_env(), false);
        let other = Addr::unchecked("addr_other");
        let contributors = vec![Addr::unchecked("addr1"), Addr::unchecked("addr2")];

        // Test adding contributors by gp
        let response = handle(deps.as_mut(), gp.clone(), contributors.clone()).unwrap();
        assert_eq!(response.messages.len(), 0);
        assert_eq!(response.attributes.len(), 2);
        assert_eq!(response.attributes[0].key, "action");
        assert_eq!(response.attributes[0].value, "whitelist_removed");
        assert_eq!(response.attributes[1].key, "whitelist_address_removed");
        assert_eq!(response.attributes[1].value, "addr1,addr2");

        // Test adding contributors by someone else
        let result = handle(deps.as_mut(), other.clone(), contributors.clone());
        assert!(result.is_err());
        match result {
            Ok(_) => panic!("Expected error"),
            Err(err) => assert_eq!(err, ContractError::Unauthorized {}),
        }

        Ok(())
    }
}
