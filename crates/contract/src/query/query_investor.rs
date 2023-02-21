use cosmwasm_std::{to_binary, Addr, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryInvestorResponse},
    storage,
};

pub fn query_investor(storage: &dyn Storage, lp: Addr) -> ProvQueryResponse {
    let commitment = storage::commits::get(storage, lp.clone())?;
    let paid_in_capital = storage::paid_in_capital::get(storage, lp);
    let response = QueryInvestorResponse {
        commitment,
        paid_in_capital,
    };
    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::mock_env, Addr};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        contract::query,
        core::msg::{QueryInvestorResponse, QueryMsg},
        execute::settlement::commitment::CommitmentState,
        util::testing::{
            create_testing_commitments, instantiate_contract, test_security_commitments,
        },
    };

    #[test]
    fn test_query_investor() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        create_testing_commitments(&mut deps);
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::QueryInvestor {
                investor: Addr::unchecked("lp1"),
            },
        )
        .unwrap();
        let value: QueryInvestorResponse = from_binary(&res).unwrap();
        assert_eq!(test_security_commitments(), value.paid_in_capital);
        assert_eq!(Addr::unchecked("lp1"), value.commitment.lp);
        assert_eq!(test_security_commitments(), value.commitment.commitments);
        // They are not settled until it is withdrawn
        assert_eq!(CommitmentState::ACCEPTED, value.commitment.state);
    }
}
