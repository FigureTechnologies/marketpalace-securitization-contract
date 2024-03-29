use cosmwasm_std::Coin;

use crate::{
    core::{error::ContractError, msg::ExecuteMsg},
    util::validate::{Validate, ValidateResult},
};

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        match self {
            ExecuteMsg::ProposeCommitment { securities } => {
                if securities.is_empty() {
                    return Err(ContractError::EmptySecurityCommitmentList {});
                }
                if securities
                    .iter()
                    .any(|commitment| commitment.amount.is_zero())
                {
                    return Err(ContractError::InvalidSecurityCommitmentAmount {});
                }
            }
            ExecuteMsg::AcceptCommitment { commitments } => {
                if commitments.is_empty() {
                    return Err(ContractError::EmptyAcceptedCommitmentList {});
                }
            }
            ExecuteMsg::DepositCommitment { securities } => {
                if securities.is_empty() {
                    return Err(ContractError::EmptySecurityCommitmentList {});
                }
                if securities
                    .iter()
                    .any(|commitment| commitment.amount.is_zero())
                {
                    return Err(ContractError::InvalidSecurityCommitmentAmount {});
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn validate_msg_funds(&self, funds: &[Coin]) -> ValidateResult {
        if !funds.is_empty() {
            return Err(ContractError::UnexpectedFunds {});
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin, Uint128};

    use crate::{
        core::{
            error::ContractError,
            msg::ExecuteMsg,
            security::{AcceptedCommitment, SecurityCommitment},
        },
        util::validate::Validate,
    };

    #[test]
    fn test_propose_has_securities() {
        let propose = ExecuteMsg::ProposeCommitment { securities: vec![] };
        let output = propose.validate().unwrap_err();
        let expected = ContractError::EmptySecurityCommitmentList {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_propose_has_valid_security_amounts() {
        let propose = ExecuteMsg::ProposeCommitment {
            securities: vec![
                SecurityCommitment {
                    name: "test".to_string(),
                    amount: Uint128::new(0),
                },
                SecurityCommitment {
                    name: "test2".to_string(),
                    amount: Uint128::new(0),
                },
            ],
        };
        let output = propose.validate().unwrap_err();
        let expected = ContractError::InvalidSecurityCommitmentAmount {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_valid_propose() {
        let propose = ExecuteMsg::ProposeCommitment {
            securities: vec![SecurityCommitment {
                name: "test".to_string(),
                amount: Uint128::new(5),
            }],
        };
        propose.validate().expect("propose should pass validation");
    }

    #[test]
    fn test_invalid_propose_when_at_least_one_has_zero() {
        let propose = ExecuteMsg::ProposeCommitment {
            securities: vec![
                SecurityCommitment {
                    name: "test".to_string(),
                    amount: Uint128::new(5),
                },
                SecurityCommitment {
                    name: "test2".to_string(),
                    amount: Uint128::new(0),
                },
            ],
        };
        let output = propose.validate().unwrap_err();
        let expected = ContractError::InvalidSecurityCommitmentAmount {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_accept_has_commitments() {
        let msg = ExecuteMsg::AcceptCommitment {
            commitments: vec![],
        };
        let output = msg.validate().unwrap_err();
        let expected = ContractError::EmptyAcceptedCommitmentList {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_valid_accept() {
        let msg = ExecuteMsg::AcceptCommitment {
            commitments: vec![AcceptedCommitment {
                lp: Addr::unchecked("address"),
                securities: vec![],
            }],
        };
        msg.validate().expect("accept should pass validation");
    }

    #[test]
    fn test_deposit_has_securities() {
        let msg = ExecuteMsg::DepositCommitment { securities: vec![] };
        let output = msg.validate().unwrap_err();
        let expected = ContractError::EmptySecurityCommitmentList {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_deposit_has_valid_security_amounts() {
        let msg = ExecuteMsg::DepositCommitment {
            securities: vec![SecurityCommitment {
                name: "test".to_string(),
                amount: Uint128::new(0),
            }],
        };
        let output = msg.validate().unwrap_err();
        let expected = ContractError::InvalidSecurityCommitmentAmount {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_invalid_deposit_with_at_least_one_zero() {
        let msg = ExecuteMsg::DepositCommitment {
            securities: vec![
                SecurityCommitment {
                    name: "test".to_string(),
                    amount: Uint128::new(5),
                },
                SecurityCommitment {
                    name: "test2".to_string(),
                    amount: Uint128::new(0),
                },
            ],
        };
        let output = msg.validate().unwrap_err();
        let expected = ContractError::InvalidSecurityCommitmentAmount {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_valid_withdraw() {
        let msg = ExecuteMsg::WithdrawCommitment {
            lp: Addr::unchecked("lp"),
        };
        msg.validate().expect("withdraw should pass validation");
    }

    #[test]
    fn test_invalid_msg_funds_deposit() {
        let msg = ExecuteMsg::DepositCommitment {
            securities: vec![SecurityCommitment {
                name: "test".to_string(),
                amount: Uint128::new(5),
            }],
        };
        let funds = vec![Coin {
            denom: "denom".to_string(),
            amount: Uint128::new(5),
        }];
        let output = msg.validate_msg_funds(&funds).unwrap_err();
        let expected = ContractError::UnexpectedFunds {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_valid_msg_funds_deposit() {
        let msg = ExecuteMsg::DepositCommitment {
            securities: vec![SecurityCommitment {
                name: "test".to_string(),
                amount: Uint128::new(5),
            }],
        };
        let funds = vec![];
        msg.validate_msg_funds(&funds)
            .expect("should pass with valid funds");
    }

    #[test]
    fn test_other_msgs_should_not_have_funds() {
        let msg = ExecuteMsg::WithdrawCommitment {
            lp: Addr::unchecked("lp"),
        };
        let msg2 = ExecuteMsg::WithdrawAllCommitments {};
        let funds = vec![Coin {
            denom: "denom".to_string(),
            amount: Uint128::new(5),
        }];
        let output = msg.validate_msg_funds(&funds).unwrap_err();
        let expected = ContractError::UnexpectedFunds {}.to_string();
        assert_eq!(expected, output.to_string());

        let output = msg2.validate_msg_funds(&funds).unwrap_err();
        let expected = ContractError::UnexpectedFunds {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_other_msgs_are_valid_without_funds() {
        let msg = ExecuteMsg::WithdrawCommitment {
            lp: Addr::unchecked("lp"),
        };
        let msg2 = ExecuteMsg::WithdrawAllCommitments {};
        let funds = vec![];
        msg.validate_msg_funds(&funds)
            .expect("should pass with no funds");
        msg2.validate_msg_funds(&funds)
            .expect("should pass with no funds");
    }
}
