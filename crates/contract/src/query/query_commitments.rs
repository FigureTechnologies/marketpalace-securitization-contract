use cosmwasm_std::{to_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryCommitmentsResponse},
    execute::settlement::commitment::CommitmentState,
    storage,
};

pub fn handle(storage: &dyn Storage, commitment_state: CommitmentState) -> ProvQueryResponse {
    let commitments = storage::commits::get_with_state(storage, commitment_state);
    let response = QueryCommitmentsResponse { commitments };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::mock_env, Addr};
    use provwasm_mocks::mock_provenance_dependencies;
    use crate::{
        contract::query,
        core::msg::{QueryCommitmentsResponse, QueryMsg},
        execute::settlement::commitment::CommitmentState,
        util::testing::{
            create_testing_commitments, instantiate_contract, test_security_commitments,
        },
    };

    #[test]
    fn test_pending_commitments() {
        let mut deps = mock_provenance_dependencies();
        instantiate_contract(deps.as_mut()).expect("Contract should instantiate.");
        create_testing_commitments(&mut deps);
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::QueryCommitments {
                commitment_state: CommitmentState::PENDING,
            },
        )
        .unwrap();
        let value: QueryCommitmentsResponse = from_binary(&res).unwrap();
        let expected_pending = test_security_commitments();
        assert_eq!(3, value.commitments.len());

        for i in 0..value.commitments.len() {
            assert_eq!(
                Addr::unchecked(format!("lp{}", i + 4)),
                value.commitments[i].lp
            );
            assert_eq!(expected_pending, value.commitments[i].commitments);
            assert_eq!(CommitmentState::PENDING, value.commitments[i].state);
        }
    }

    #[test]
    fn test_accepted_commitments() {
        let mut deps = mock_provenance_dependencies();
        instantiate_contract(deps.as_mut()).expect("Contract should instantiate.");
        create_testing_commitments(&mut deps);
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::QueryCommitments {
                commitment_state: CommitmentState::ACCEPTED,
            },
        )
        .unwrap();
        let value: QueryCommitmentsResponse = from_binary(&res).unwrap();
        let expected_accepted = test_security_commitments();
        assert_eq!(3, value.commitments.len());

        for i in 0..value.commitments.len() {
            assert_eq!(
                Addr::unchecked(format!("lp{}", i + 1)),
                value.commitments[i].lp
            );
            assert_eq!(expected_accepted, value.commitments[i].commitments);
            assert_eq!(CommitmentState::ACCEPTED, value.commitments[i].state);
        }
    }

    #[test]
    fn test_settled_commitments() {
        let mut deps = mock_provenance_dependencies();
        instantiate_contract(deps.as_mut()).expect("Contract should instantiate.");
        create_testing_commitments(&mut deps);
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::QueryCommitments {
                commitment_state: CommitmentState::SETTLED,
            },
        )
        .unwrap();
        let value: QueryCommitmentsResponse = from_binary(&res).unwrap();
        let expected_accepted = test_security_commitments();
        assert_eq!(1, value.commitments.len());

        for i in 0..value.commitments.len() {
            assert_eq!(
                Addr::unchecked(format!("lp{}", i + 7)),
                value.commitments[i].lp
            );
            assert_eq!(expected_accepted, value.commitments[i].commitments);
            assert_eq!(CommitmentState::SETTLED, value.commitments[i].state);
        }
    }
}
