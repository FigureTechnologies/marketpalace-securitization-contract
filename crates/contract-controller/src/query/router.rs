use cosmwasm_std::Env;

use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryMsg,
};

use super::{query_contracts, query_state, query_version};

pub fn route(deps: ProvDeps, _env: Env, msg: QueryMsg) -> ProvQueryResponse {
    match msg {
        QueryMsg::QueryVersion {} => query_version::handle(deps.storage),
        QueryMsg::QueryState {} => query_state::handle(deps.storage),
        QueryMsg::QueryContracts {} => query_contracts::handle(deps.storage),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::mock_env};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::msg::{QueryContractsResponse, QueryMsg, QueryStateResponse, QueryVersionResponse},
        query,
        util::testing::instantiate_contract,
    };

    #[test]
    fn test_query_version_has_correct_response() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let message = QueryMsg::QueryVersion {};
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let res = query::router::route(deps.as_ref(), env, message).unwrap();
        let _: QueryVersionResponse = from_binary(&res).unwrap();
    }

    #[test]
    fn test_query_state_has_correct_response() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let message = QueryMsg::QueryState {};
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let res = query::router::route(deps.as_ref(), env, message).unwrap();
        let _: QueryStateResponse = from_binary(&res).unwrap();
    }

    #[test]
    fn test_query_contracts_has_correct_response() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let message = QueryMsg::QueryContracts {};
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let res = query::router::route(deps.as_ref(), env, message).unwrap();
        let _: QueryContractsResponse = from_binary(&res).unwrap();
    }
}
