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
            ExecuteMsg::WithdrawCommitments {} => {}
        };
        Ok(())
    }

    fn validate_msg_funds(&self, funds: &[Coin]) -> ValidateResult {
        match self {
            ExecuteMsg::DepositCommitment { securities: _ } => {
                if funds.is_empty() {
                    return Err(ContractError::MissingFunds {});
                }
                Ok(())
            }
            _ => {
                if !funds.is_empty() {
                    return Err(ContractError::UnexpectedFunds {});
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin, Uint128};

    use crate::{
        core::{error::ContractError, msg::ExecuteMsg, security::SecurityCommitment},
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
            securities: vec![SecurityCommitment {
                name: "test".to_string(),
                amount: Uint128::new(0),
            }],
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
            commitments: vec![Addr::unchecked("address")],
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
    fn test_valid_deposit() {
        let msg = ExecuteMsg::DepositCommitment {
            securities: vec![SecurityCommitment {
                name: "test".to_string(),
                amount: Uint128::new(5),
            }],
        };
        msg.validate().expect("deposit should pass validation");
    }

    #[test]
    fn test_valid_withdraw() {
        let msg = ExecuteMsg::WithdrawCommitments {};
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
        let funds = vec![];
        let output = msg.validate_msg_funds(&funds).unwrap_err();
        let expected = ContractError::MissingFunds {}.to_string();
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
        let funds = vec![Coin {
            denom: "denom".to_string(),
            amount: Uint128::new(5),
        }];
        msg.validate_msg_funds(&funds)
            .expect("should pass with valid funds");
    }

    #[test]
    fn test_other_msgs_should_not_have_funds() {
        let msg = ExecuteMsg::WithdrawCommitments {};
        let funds = vec![Coin {
            denom: "denom".to_string(),
            amount: Uint128::new(5),
        }];
        let output = msg.validate_msg_funds(&funds).unwrap_err();
        let expected = ContractError::UnexpectedFunds {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_other_msgs_are_valid_without_funds() {
        let msg = ExecuteMsg::WithdrawCommitments {};
        let funds = vec![];
        msg.validate_msg_funds(&funds)
            .expect("should pass with no funds");
    }
}
