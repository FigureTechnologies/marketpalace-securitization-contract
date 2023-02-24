use cosmwasm_std::Env;

use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryMsg,
};

mod query_investor;
mod query_pending_commitments;
mod query_securitizations;
mod query_state;
mod query_version;
mod validate;

pub fn handle(deps: ProvDeps, _env: Env, msg: QueryMsg) -> ProvQueryResponse {
    match msg {
        QueryMsg::QueryInvestor { investor } => {
            query_investor::query_investor(deps.storage, investor)
        }
        QueryMsg::QueryPendingCommitments {} => {
            query_pending_commitments::query_pending_commitments(deps.storage)
        }
        QueryMsg::QuerySecuritizations { securities } => {
            query_securitizations::query_securitizations(deps.storage, securities)
        }
        QueryMsg::QueryState {} => query_state::query_state(deps.storage),
        QueryMsg::QueryVersion {} => query_version::query_version(deps.storage),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::mock_env, Addr};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::msg::{
            QueryInvestorResponse, QueryPendingCommitmentsResponse, QuerySecuritizationsResponse,
            QueryStateResponse, QueryVersionResponse,
        },
        util,
    };

    use super::handle;

    #[test]
    fn tests_query_investor_has_correct_response() {
        let mut deps = mock_dependencies(&[]);
        let msg = crate::core::msg::QueryMsg::QueryInvestor {
            investor: Addr::unchecked("lp1"),
        };
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp1").unwrap();
        let bin = handle(deps.as_ref(), mock_env(), msg).unwrap();
        let _: QueryInvestorResponse = from_binary(&bin).unwrap();
    }

    #[test]
    fn tests_query_pending_commits_has_correct_response() {
        let mut deps = mock_dependencies(&[]);
        let msg = crate::core::msg::QueryMsg::QueryPendingCommitments {};
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp1").unwrap();
        let bin = handle(deps.as_ref(), mock_env(), msg).unwrap();
        let _: QueryPendingCommitmentsResponse = from_binary(&bin).unwrap();
    }

    #[test]
    fn tests_query_state_has_correct_response() {
        let mut deps = mock_dependencies(&[]);
        let msg = crate::core::msg::QueryMsg::QueryState {};
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp1").unwrap();
        let bin = handle(deps.as_ref(), mock_env(), msg).unwrap();
        let _: QueryStateResponse = from_binary(&bin).unwrap();
    }

    #[test]
    fn tests_query_version_has_correct_response() {
        let mut deps = mock_dependencies(&[]);
        let msg = crate::core::msg::QueryMsg::QueryVersion {};
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp1").unwrap();
        let bin = handle(deps.as_ref(), mock_env(), msg).unwrap();
        let _: QueryVersionResponse = from_binary(&bin).unwrap();
    }

    #[test]
    fn tests_query_securitizations_has_correct_response() {
        let mut deps = mock_dependencies(&[]);
        let msg = crate::core::msg::QueryMsg::QuerySecuritizations {
            securities: vec!["Security1".to_string()],
        };
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        let bin = handle(deps.as_ref(), mock_env(), msg).unwrap();
        let _: QuerySecuritizationsResponse = from_binary(&bin).unwrap();
    }
}
