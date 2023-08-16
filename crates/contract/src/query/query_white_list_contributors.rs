use crate::core::aliases::ProvQueryResponse;
use crate::core::msg::{QueryLoanPoolContributorsResponse};
use cosmwasm_std::{to_binary, Storage};
use crate::storage::whitelist_contributors_store::get_whitelist_contributors;


/// Handles the querying of whitelist contributors from your storage,
/// and returns a ProvQueryResponse with the result.
///
/// # Arguments
///
/// * `storage` - A dynamic reference to an object that implements the Storage trait.
///    It should be capable of storing and retrieving data objects.
///
/// # Returns
///
/// * If successful, this function returns a `Result` wrapping a `ProvQueryResponse`
///    which encodes the list of contributors in a binary format. If an error occurs
///    while getting the contributors or serializing them, the error will be returned.
///
/// # Examples
///
/// ```rust
/// use your_storage_lib::Storage;
///
/// // assuming you have a valid storage object
/// let storage = get_storage(); // Placeholder function
/// let response = handle(&storage);
/// match response {
///    Ok(data) => println!("Data: {:?}", data),
///    Err(e) => println!("Error: {:?}", e),
/// }
/// ```
///
/// # Errors
///
/// Will return an error if either loading the contributors from the storage fails
/// or serialization of the `QueryLoanPoolContributorsResponse` struct into a binary format fails.
///
/// # Panics
///
/// This function should not panic.
///
/// # Safety
///
/// This function assumes the storage passed as parameter correctly and safely implements the `Storage` trait.
pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    Ok(to_binary(
        &QueryLoanPoolContributorsResponse {
            contributors: get_whitelist_contributors(storage),
        },
    )?)
}

#[cfg(test)]
mod tests {
    use crate::contract::query;
    use crate::core::msg::{QueryLoanPoolContributorsResponse, QueryMsg};
    use crate::execute::settlement::whitelist_loanpool_contributors::handle as whitelist_loanpool_handle;
    use crate::util::testing::instantiate_contract;
    use cosmwasm_std::testing::mock_info;
    use cosmwasm_std::{from_binary, testing::mock_env, Addr};
    use provwasm_mocks::mock_dependencies;

    #[test]
    fn test_all_whitelist_success() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let env = mock_env();
        let info = mock_info("contributor", &[]);
        let info_white_list = mock_info("gp", &[]);
        let addr_contributor = Addr::unchecked("contributor");
        let white_list_addr = vec![addr_contributor.clone()];
        let whitelist_result =
            whitelist_loanpool_handle(deps.as_mut(), info_white_list.sender, white_list_addr);
        assert!(whitelist_result.is_ok());

        //query all states
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryLoanPoolContributors {}).unwrap();
        let value: QueryLoanPoolContributorsResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.contributors.len());
    }

    #[test]
    fn test_all_contributors_none_exists() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");

        //query all states
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryLoanPoolContributors {}).unwrap();
        let value: QueryLoanPoolContributorsResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.contributors.len());
    }
}
