use contract::core::{msg::ExecuteMsg, security::SecurityCommitment};

use crate::{security, user};

pub fn create() {
    let message = ExecuteMsg::ProposeCommitment {
        securities: collect_commitments(),
    };
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}

fn collect_commitments() -> Vec<SecurityCommitment> {
    let mut commitments = vec![];

    let commitment_count = user::get_int_input("Enter number of security commitments: ");

    for _ in 0..commitment_count {
        commitments.push(security::create_commitment_from_input());
    }

    commitments
}
