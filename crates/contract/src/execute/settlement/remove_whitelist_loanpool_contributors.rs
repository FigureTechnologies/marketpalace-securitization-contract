use cosmwasm_std::{Addr, Env, Response, Storage};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    storage::{
        state::{self},
    },
};
use crate::storage::whitelist_contributors_store::{remove_contributors, save_contributors};


pub fn handle(mut deps: ProvDepsMut, sender: Addr, contributors: Vec<Addr>) -> ProvTxResponse {
    let state = state::get(deps.storage)?;
    if sender != state.gp {
        // only gp can add whitelisted contributor
        return Err(ContractError::Unauthorized {});
    }

    remove_loan_pool_contributors(&mut deps, contributors)
}
pub fn remove_loan_pool_contributors(
    deps: &mut ProvDepsMut,
    loan_pool_contributors: Vec<Addr>,
) -> ProvTxResponse {
    remove_contributors(deps.storage, loan_pool_contributors.clone())?;

    // Converting Vec<Addr> to Vec<String>
    let contributors_as_str: Vec<String> = loan_pool_contributors.into_iter().map(|addr| addr.to_string()).collect();
    // Joining Vec<String> into a single String
    let contributors_str = contributors_as_str.join(",");

    let response = Response::new()
        .add_attribute("action", "whitelist_removed")
        .add_attribute("address_removed", contributors_str);

    Ok(response)
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env};
    use cosmwasm_std::{Addr, Env, Response, StdResult};
    use provwasm_mocks::mock_dependencies;
    use crate::util::testing::create_test_state;

    #[test]
    fn test_add_contributors() -> StdResult<()> {
        let gp = Addr::unchecked("gp");

        let mut deps = mock_dependencies(&[]);
        create_test_state(&mut deps, &mock_env(), false);
        let env = mock_env();
        let other = Addr::unchecked("addr_other");
        let contributors = vec![Addr::unchecked("addr1"), Addr::unchecked("addr2")];


        // Test adding contributors by gp
        let response = handle(deps.as_mut(),  gp.clone(), contributors.clone()).unwrap();
        assert_eq!(response.messages.len(), 0);
        assert_eq!(response.attributes.len(), 2);
        assert_eq!(response.attributes[0].key, "action");
        assert_eq!(response.attributes[0].value, "whitelist_added");
        assert_eq!(response.attributes[1].key, "address_whitelisted");
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


