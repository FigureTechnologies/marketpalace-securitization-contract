use cosmwasm_std::{to_json_binary, Storage};
use cw2::get_contract_version;

use crate::core::{aliases::ProvQueryResponse, msg::QueryVersionResponse};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let response = QueryVersionResponse {
        contract_version: get_contract_version(storage)?,
    };
    Ok(to_json_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, testing::mock_env};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{
            constants::{CONTRACT_NAME, CONTRACT_VERSION},
            msg::QueryVersionResponse,
        },
        util::testing::instantiate_contract,
    };

    use super::handle;

    #[test]
    fn test_query_version_has_correct_version() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let _ = instantiate_contract(deps.as_mut(), env).unwrap();
        let bin_response = handle(&deps.storage).unwrap();
        let response: QueryVersionResponse = from_json(&bin_response).unwrap();
        assert_eq!(response.contract_version.version, CONTRACT_VERSION);
        assert_eq!(response.contract_version.contract, CONTRACT_NAME);
    }
}
