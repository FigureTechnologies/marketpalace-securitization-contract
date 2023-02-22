use contract::core::{msg::ExecuteMsg, security::SecurityCommitment};

use crate::{security, user};

pub fn create() -> ExecuteMsg {
    ExecuteMsg::ProposeCommitment {
        securities: collect_commitments(),
    }
}

fn collect_commitments() -> Vec<SecurityCommitment> {
    let mut commitments = vec![];

    let commitment_count = user::get_int_input("Enter number of security commitments: ");

    for _ in 0..commitment_count {
        commitments.push(security::create_commitment_from_input());
    }

    commitments
}
