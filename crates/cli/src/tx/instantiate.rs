use contract::core::{fee::Fee, rules::InvestmentVehicleRule, security::Security};
use cosmwasm_std::{Addr, Coin, Uint64};

use crate::{security, user};

pub fn create(gp: &str, denom: String) {
    let securities = collect_securities(&denom);
    let message = contract::core::msg::InstantiateMsg {
        gp: Addr::unchecked(gp),
        securities: securities,
        capital_denom: denom,
        rules: vec![InvestmentVehicleRule::SettlementTime(Uint64::new(
            1678975183,
        ))],
        fee: Some(Fee {
            recipient: Addr::unchecked("receiver"),
            amount: Coin::new(100, "nhash"),
        }),
    };
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}

fn collect_securities(capital_denom: &String) -> Vec<Security> {
    let mut securities = vec![];

    let security_count = user::get_int_input("Enter number of securities: ");

    for _ in 0..security_count {
        securities.push(security::create_from_input(&capital_denom));
    }

    securities
}
