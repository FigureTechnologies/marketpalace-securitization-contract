use cosmwasm_std::{to_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryPendingCommitmentsResponse},
    storage,
};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let commitments = storage::commits::get_pending(storage);
    let response = QueryPendingCommitmentsResponse { commitments };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::mock_env, Addr};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        contract::query,
        core::msg::{QueryMsg, QueryPendingCommitmentsResponse},
        execute::settlement::commitment::CommitmentState,
        util::testing::{
            create_testing_commitments, instantiate_contract, test_security_commitments,
        },
    };

    #[test]
    fn test_pending_commitments() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("Contract should instantiate.");
        create_testing_commitments(&mut deps);
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::QueryPendingCommitments {},
        )
        .unwrap();
        let value: QueryPendingCommitmentsResponse = from_binary(&res).unwrap();
        let expected_security_commitments = test_security_commitments();
        assert_eq!(3, value.commitments.len());

        for i in 0..value.commitments.len() {
            assert_eq!(
                Addr::unchecked(format!("lp{}", i + 4)),
                value.commitments[i].lp
            );
            assert_eq!(
                expected_security_commitments,
                value.commitments[i].commitments
            );
            assert_eq!(CommitmentState::PENDING, value.commitments[i].state);
        }
    }
}
