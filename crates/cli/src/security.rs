use contract::core::security::{
    FundSecurity, PrimarySecurity, Security, SecurityCommitment, SecurityType, TrancheSecurity,
};
use cosmwasm_std::{Coin, Uint128};

use crate::user;

pub fn create_from_input(capital_denom: &str) -> Security {
    let name = user::get_input("Enter security name: ");
    let security_type =
        match user::get_input("Enter security type (0 Tranche, 1 Primary, 2 Fund): ").as_str() {
            "0" => SecurityType::Tranche(TrancheSecurity {}),
            "1" => SecurityType::Primary(PrimarySecurity {}),
            "2" => SecurityType::Fund(FundSecurity {}),
            _ => panic!("Unexpected security type"),
        };
    let amount = user::get_int_input("Enter amount of security: ");
    let minimum_amount = user::get_int_input("Enter minimum amount of security: ");
    let price = user::get_int_input("Enter price per unit of security: ");
    let price_per_unit = Coin::new(price, capital_denom);

    Security {
        name,
        amount: Uint128::new(amount),
        security_type,
        minimum_amount: Uint128::new(minimum_amount),
        price_per_unit,
    }
}

pub fn create_commitment_from_input() -> SecurityCommitment {
    let name = user::get_input("Enter security name: ");
    let amount = user::get_int_input("Enter amount of security: ");
    SecurityCommitment {
        name,
        amount: Uint128::new(amount),
    }
}
