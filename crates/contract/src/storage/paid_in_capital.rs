use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::Map;

use crate::core::{
    constants::PAID_IN_CAPITAL_KEY, error::ContractError, security::SecurityCommitment,
};

pub const PAID_IN_CAPITAL: Map<Addr, Vec<SecurityCommitment>> = Map::new(PAID_IN_CAPITAL_KEY);

pub fn get(storage: &dyn Storage, lp: Addr) -> Vec<SecurityCommitment> {
    let capital = PAID_IN_CAPITAL.load(storage, lp);
    match capital {
        Ok(capital) => capital,
        Err(_) => vec![],
    }
}

pub fn remove(storage: &mut dyn Storage, lp: Addr) {
    PAID_IN_CAPITAL.remove(storage, lp);
}

pub fn has_lp(storage: &dyn Storage, lp: Addr) -> bool {
    PAID_IN_CAPITAL.has(storage, lp)
}

pub fn set(
    storage: &mut dyn Storage,
    lp: Addr,
    commitments: &Vec<SecurityCommitment>,
) -> Result<(), ContractError> {
    Ok(PAID_IN_CAPITAL.save(storage, lp, commitments)?)
}

pub fn add_payment(
    storage: &mut dyn Storage,
    lp: Addr,
    deposit: Vec<SecurityCommitment>,
) -> Result<(), ContractError> {
    PAID_IN_CAPITAL.update(
        storage,
        lp,
        |already_committed| -> StdResult<Vec<SecurityCommitment>> {
            match already_committed {
                None => Ok(deposit),
                Some(mut already_committed) => {
                    for deposit_security in &deposit {
                        add_security_commitment(deposit_security, &mut already_committed);
                    }
                    Ok(already_committed)
                }
            }
        },
    )?;
    Ok(())
}

// The purpose of this function is to add new_commitment to commitments.
// We do this by finding the security commitment that has the same name as new_commitment,
// and then we add the new_commitment.amount to the commitment.amount.
//
// Note this modifies commitments
fn add_security_commitment(
    new_commitment: &SecurityCommitment,
    commitments: &mut [SecurityCommitment],
) {
    for commitment in commitments.iter_mut() {
        if commitment.name == new_commitment.name {
            commitment.amount += new_commitment.amount;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint128};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::security::SecurityCommitment,
        execute::settlement::commitment::Commitment,
        storage::paid_in_capital::{add_security_commitment, get, remove, set, PAID_IN_CAPITAL},
        util::testing::SettlementTester,
    };

    use super::add_payment;

    #[test]
    fn test_add_security_commitment_with_empty() {
        let mut settlement_tester = SettlementTester::new();
        settlement_tester.create_security_commitments(1);
        let new_commitment = settlement_tester.security_commitments[0].clone();
        let mut commitments = vec![];

        add_security_commitment(&new_commitment, &mut commitments);
        assert_eq!(0, commitments.len());
    }

    #[test]
    fn test_remove() {
        let mut deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp.clone(), vec![]);
        set(deps.as_mut().storage, commitment.lp.clone(), &vec![]).unwrap();
        remove(deps.as_mut().storage, lp.clone());
        assert_eq!(false, PAID_IN_CAPITAL.has(&deps.storage, lp));
    }

    #[test]
    fn test_has() {
        let mut deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp.clone(), vec![]);
        assert_eq!(false, super::has_lp(&deps.storage, lp.clone()));
        set(deps.as_mut().storage, commitment.lp.clone(), &vec![]).unwrap();
        assert_eq!(true, super::has_lp(&deps.storage, lp));
    }

    #[test]
    fn test_add_security_commitment_updates_first_capital() {
        let new_commitment = SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(5),
        };
        let mut commitments = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(7),
            },
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(5),
            },
        ];

        add_security_commitment(&new_commitment, &mut commitments);
        assert_eq!(2, commitments.len());
        assert_eq!(Uint128::new(12), commitments[0].amount);
        assert_eq!(Uint128::new(5), commitments[1].amount);
    }

    #[test]
    fn test_add_security_commitment_ignores_invalid_name() {
        let new_commitment = SecurityCommitment {
            name: "Security1".to_string(),
            amount: Uint128::new(5),
        };
        let mut commitments = vec![
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(7),
            },
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(5),
            },
        ];

        add_security_commitment(&new_commitment, &mut commitments);
        assert_eq!(2, commitments.len());
        assert_eq!(Uint128::new(7), commitments[0].amount);
        assert_eq!(Uint128::new(10), commitments[1].amount);
    }

    #[test]
    fn test_get_set() {
        let mut deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");

        let commitments = vec![
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(7),
            },
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(5),
            },
        ];

        set(deps.as_mut().storage, lp.clone(), &commitments).unwrap();
        let obtained = get(&deps.storage, lp);

        assert_eq!(commitments, obtained);
    }

    #[test]
    fn test_get_invalid() {
        let deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");
        assert_eq!(Vec::<SecurityCommitment>::new(), get(&deps.storage, lp));
    }

    #[test]
    fn add_payment_new_entry() {
        let mut deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");
        let commitments = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(5),
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(7),
            },
        ];
        add_payment(deps.as_mut().storage, lp.clone(), commitments.clone()).unwrap();
        let obtained = get(&deps.storage, lp);

        assert_eq!(commitments, obtained);
    }

    #[test]
    fn add_payment_update_entry() {
        let mut deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");
        let commitments = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(5),
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(7),
            },
        ];
        add_payment(deps.as_mut().storage, lp.clone(), commitments.clone()).unwrap();
        add_payment(deps.as_mut().storage, lp.clone(), commitments.clone()).unwrap();
        let obtained = get(&deps.storage, lp);

        let expected = vec![
            SecurityCommitment {
                name: "Security1".to_string(),
                amount: Uint128::new(10),
            },
            SecurityCommitment {
                name: "Security2".to_string(),
                amount: Uint128::new(14),
            },
        ];

        assert_eq!(expected, obtained);
    }
}
