use cosmwasm_std::{to_json_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryContractAddressResponse},
    storage,
};

pub fn handle(storage: &dyn Storage, uuid: String) -> ProvQueryResponse {
    let response = QueryContractAddressResponse {
        contract: storage::uuid::get(storage, &uuid)?,
    };
    Ok(to_json_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, testing::mock_env, Addr};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::msg::QueryContractAddressResponse,
        query::query_contract_address::handle,
        util::testing::{add_contracts, create_admin_deps, instantiate_contract},
    };

    #[test]
    fn test_query_contracts_handles_invalid() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let _ = instantiate_contract(deps.as_mut(), env).unwrap();
        handle(&deps.storage, "bad_address".to_string()).unwrap_err();
    }

    #[test]
    fn test_query_contracts() {
        let mut deps = create_admin_deps(&[]);
        let env = mock_env();
        let _ = instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        add_contracts(deps.as_mut(), env).unwrap();
        let bin_response = handle(&deps.storage, "uuid1".to_string()).unwrap();
        let response: QueryContractAddressResponse = from_json(&bin_response).unwrap();
        assert_eq!(Addr::unchecked("contract1"), response.contract);
    }
}
