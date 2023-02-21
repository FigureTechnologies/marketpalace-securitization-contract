use cosmwasm_std::{to_binary, Storage};
use cw2::get_contract_version;

use crate::core::{aliases::ProvQueryResponse, msg::QueryVersionResponse};

pub fn query_version(storage: &dyn Storage) -> ProvQueryResponse {
    let response = QueryVersionResponse {
        contract_version: get_contract_version(storage)?,
    };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::mock_env};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        contract::query,
        core::msg::{QueryMsg, QueryVersionResponse},
        util::testing::instantiate_contract,
    };

    #[test]
    fn test_has_correct_version() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryVersion {}).unwrap();
        let value: QueryVersionResponse = from_binary(&res).unwrap();
        assert_eq!(
            "marketpalace-securitization-contract".to_string(),
            value.contract_version.contract
        );
        assert_eq!("1.0.0".to_string(), value.contract_version.version);
    }
}
