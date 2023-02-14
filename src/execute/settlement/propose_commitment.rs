use cosmwasm_std::{Addr, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        security::SecurityCommitment,
    },
    storage::{
        commits::{self},
        securities::SECURITIES_MAP,
    },
};

use super::commitment::Commitment;

pub fn handle(deps: ProvDepsMut, lp: Addr, commitments: Vec<SecurityCommitment>) -> ProvTxResponse {
    for commitment in &commitments {
        let security = SECURITIES_MAP.load(deps.storage, commitment.name.clone())?;
        if commitment.amount < security.minimum_amount {
            return Err(crate::core::error::ContractError::InvalidSecurityCommitmentAmount {});
        }
    }

    let commitment = Commitment::new(lp, commitments);

    commits::set(deps.storage, &commitment)?;
    Ok(Response::new())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{Addr, Coin};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::{
            error::ContractError,
            security::{FundSecurity, Security},
        },
        execute::{propose_commitment::handle, settlement::commitment::CommitmentState},
        storage::{
            commits::{self},
            securities::SECURITIES_MAP,
        },
        util::testing::SettlementTester,
    };

    #[test]
    fn test_minimums_are_met() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let commitments = settlement_tester.security_commitments.clone();
        SECURITIES_MAP
            .save(
                &mut deps.storage,
                commitments[0].name.clone(),
                &Security {
                    name: commitments[0].name.clone(),
                    amount: 10,
                    security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                    minimum_amount: commitments[0].amount + 1,
                    price_per_unit: Coin::new(5, "denom".to_string()),
                },
            )
            .unwrap();
        let res = handle(deps.as_mut(), lp, commitments).unwrap_err();

        assert_eq!(
            ContractError::InvalidSecurityCommitmentAmount {}.to_string(),
            res.to_string()
        )
    }

    #[test]
    fn test_all_securities_exist() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let commitments = settlement_tester.security_commitments.clone();
        handle(deps.as_mut(), lp, commitments).unwrap_err();
    }

    #[test]
    fn test_commit_is_added_on_success() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("address");
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let commitments = settlement_tester.security_commitments.clone();
        SECURITIES_MAP
            .save(
                &mut deps.storage,
                commitments[0].name.clone(),
                &Security {
                    name: commitments[0].name.clone(),
                    amount: 10,
                    security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                    minimum_amount: commitments[0].amount,
                    price_per_unit: Coin::new(5, "denom".to_string()),
                },
            )
            .unwrap();
        handle(deps.as_mut(), lp.clone(), commitments.clone()).unwrap();

        let commitment = commits::get(&deps.storage, lp.clone()).unwrap();
        assert_eq!(commitments, commitment.commitments);
        assert_eq!(CommitmentState::PENDING, commitment.state);
        assert_eq!(lp, commitment.lp);
    }

    #[test]
    fn test_cannot_accept_security_when_total_supply_is_greater_than_amount() {
        assert!(false);
    }
}
