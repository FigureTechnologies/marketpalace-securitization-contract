use cosmwasm_std::{to_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QuerySecuritizationsResponse},
    storage::{self},
};

pub fn handle(storage: &dyn Storage, security_names: Vec<String>) -> ProvQueryResponse {
    let mut securities = vec![];

    for security in security_names {
        securities.push(storage::securities::get(storage, security)?);
    }

    let response = QuerySecuritizationsResponse { securities };

    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::mock_env};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        contract::query,
        core::msg::QuerySecuritizationsResponse,
        util::testing::{create_test_securities, instantiate_contract},
    };

    #[test]
    fn test_query_single_securitizations() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("Should instantiate");
        let res = query(
            deps.as_ref(),
            mock_env(),
            crate::core::msg::QueryMsg::QuerySecuritizations {
                securities: vec!["Security1".to_string()],
            },
        )
        .unwrap();

        let expected = create_test_securities();

        let value: QuerySecuritizationsResponse = from_binary(&res).unwrap();

        assert_eq!(1, value.securities.len());
        assert_eq!(expected[0], value.securities[0]);
    }

    #[test]
    fn test_query_multiple_securitizations() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("Should instantiate");
        let res = query(
            deps.as_ref(),
            mock_env(),
            crate::core::msg::QueryMsg::QuerySecuritizations {
                securities: vec!["Security1".to_string(), "Security2".to_string()],
            },
        )
        .unwrap();

        let expected = create_test_securities();

        let value: QuerySecuritizationsResponse = from_binary(&res).unwrap();

        assert_eq!(2, value.securities.len());
        assert_eq!(expected[0], value.securities[0]);
        assert_eq!(expected[1], value.securities[1]);
    }
}
