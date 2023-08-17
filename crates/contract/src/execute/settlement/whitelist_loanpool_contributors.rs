use cosmwasm_std::{Addr, Response};

use crate::storage::whitelist_contributors_store::save_contributors;
use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::state::{self},
};

pub fn handle(mut deps: ProvDepsMut, sender: Addr, contributors: Vec<Addr>) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        // only gp can add whitelisted contributor
        return Err(ContractError::Unauthorized {});
    }

    loan_pool_contributors(&mut deps, contributors)
}

/// This method is used to process loan pool contributors.
/// It starts by saving the contributors list using `save_contributors` function.
/// Then it first transforms each contributor `Addr` object into a String,
/// and joins them into a single `String` separated by commas.
/// Finally, it generates a `Response` with a couple of attributes: action and addresses,
/// and returns this Response. The method signature suggests it can also return
/// an error, which would likely be if saving contributors fails.
pub fn loan_pool_contributors(
    deps: &mut ProvDepsMut,
    loan_pool_contributors: Vec<Addr>,
) -> ProvTxResponse {
    save_contributors(deps.storage, loan_pool_contributors.clone())?;
    // Converting Vec<Addr> to Vec<String>
    let contributors_as_str: Vec<String> = loan_pool_contributors
        .into_iter()
        .map(|addr| addr.to_string())
        .collect();
    // Joining Vec<String> into a single String
    let contributors_str = contributors_as_str.join(",");
    let response = Response::new()
        .add_attribute("action", "whitelist_added")
        .add_attribute("addresses_whitelisted", contributors_str);
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
    fn test_add_contributors() -> StdResult<()> {
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
        assert_eq!(response.attributes[0].value, "whitelist_added");
        assert_eq!(response.attributes[1].key, "addresses_whitelisted");
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
