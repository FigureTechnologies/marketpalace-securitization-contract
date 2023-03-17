use cosmwasm_std::Coin;

use crate::{
    core::{error::ContractError, msg::InstantiateMsg},
    util::validate::{Validate, ValidateResult},
};

impl Validate for InstantiateMsg {
    fn validate(&self) -> ValidateResult {
        // Add validation checks
        if self.securities.is_empty() {
            return Err(ContractError::EmptySecurityList {});
        }

        let same_type = self
            .securities
            .iter()
            .all(|security| security.security_type == self.securities[0].security_type);
        if !same_type {
            return Err(ContractError::InvalidSecurityList {});
        }

        let minimums_are_valid = self
            .securities
            .iter()
            .all(|security| security.minimum_amount <= security.amount);
        if !minimums_are_valid {
            return Err(ContractError::InvalidSecurityList {});
        }

        let amounts_are_valid = self
            .securities
            .iter()
            .all(|security| !security.amount.is_zero());
        if !amounts_are_valid {
            return Err(ContractError::InvalidSecurityList {});
        }

        let slice = self.securities.iter().as_slice();
        let has_duplicate = (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]));
        if has_duplicate {
            return Err(ContractError::InvalidSecurityList {});
        }

        if self.capital_denom.is_empty() {
            return Err(ContractError::InvalidCapitalDenom {});
        }

        let same_denom = self
            .securities
            .iter()
            .all(|security| security.price_per_unit.denom == self.capital_denom);
        if !same_denom {
            return Err(ContractError::InvalidSecurityPriceDenom {});
        }

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
            fee::Fee,
            msg::InstantiateMsg,
            security::{FundSecurity, Security, TrancheSecurity},
        },
        util::validate::Validate,
    };

    #[test]
    fn test_success() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![
                Security {
                    name: "security 1".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
                Security {
                    name: "security 2".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
            ],
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: Some(Fee {
                recipient: Addr::unchecked("receiver"),
                amount: Coin::new(100, "nhash"),
            }),
        };
        let funds = vec![];
        msg.validate().expect("should pass validation");
        msg.validate_msg_funds(&funds)
            .expect("should not have funds");
    }

    #[test]
    fn test_invalid_security_denom() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![
                Security {
                    name: "security 1".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
                Security {
                    name: "security 2".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom2".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
            ],
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: None,
        };
        let expected = ContractError::InvalidSecurityPriceDenom {}.to_string();
        let output = msg.validate().unwrap_err();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_securities_have_same_type() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![
                Security {
                    name: "security 1".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
                Security {
                    name: "security 2".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                },
            ],
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: None,
        };
        let expected = ContractError::InvalidSecurityList {}.to_string();
        let output = msg.validate().unwrap_err();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_securities_is_not_empty() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![],
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: None,
        };
        let output = msg.validate().unwrap_err();
        let expected = ContractError::EmptySecurityList {}.to_string();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_securities_have_same_name() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![
                Security {
                    name: "security 1".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
                Security {
                    name: "security 1".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Fund(FundSecurity {}),
                },
            ],
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: None,
        };
        let expected = ContractError::InvalidSecurityList {}.to_string();
        let output = msg.validate().unwrap_err();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_securities_have_invalid_amount() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![
                Security {
                    name: "security 1".to_string(),
                    amount: Uint128::new(0),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
                Security {
                    name: "security 2".to_string(),
                    amount: Uint128::new(0),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
            ],
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: None,
        };
        let expected = ContractError::InvalidSecurityList {}.to_string();
        let output = msg.validate().unwrap_err();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_minimum_not_greater_than_amount() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![
                Security {
                    name: "security 1".to_string(),
                    amount: Uint128::new(2),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
                Security {
                    name: "security 2".to_string(),
                    amount: Uint128::new(2),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
            ],
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: None,
        };
        let expected = ContractError::InvalidSecurityList {}.to_string();
        let output = msg.validate().unwrap_err();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_has_capital_denom() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![Security {
                name: "security 1".to_string(),
                amount: Uint128::new(100),
                minimum_amount: Uint128::new(5),
                price_per_unit: Coin {
                    denom: "denom".to_string(),
                    amount: Uint128::new(5),
                },
                security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
            }],
            capital_denom: "".to_string(),
            settlement_time: None,
            fee: None,
        };
        let expected = ContractError::InvalidCapitalDenom {}.to_string();
        let output = msg.validate().unwrap_err();
        assert_eq!(expected, output.to_string());
    }

    #[test]
    fn test_no_funds() {
        let msg = InstantiateMsg {
            gp: Addr::unchecked("address"),
            securities: vec![
                Security {
                    name: "security 1".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
                Security {
                    name: "security 2".to_string(),
                    amount: Uint128::new(100),
                    minimum_amount: Uint128::new(5),
                    price_per_unit: Coin {
                        denom: "denom".to_string(),
                        amount: Uint128::new(5),
                    },
                    security_type: crate::core::security::SecurityType::Tranche(TrancheSecurity {}),
                },
            ],
            capital_denom: "denom".to_string(),
            settlement_time: None,
            fee: None,
        };
        let funds = vec![Coin {
            denom: "denom".to_string(),
            amount: Uint128::new(5),
        }];
        let output = msg.validate_msg_funds(&funds).unwrap_err();
        let expected = ContractError::UnexpectedFunds {};

        assert_eq!(expected.to_string(), output.to_string());
    }
}
