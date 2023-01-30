use cosmwasm_std::{Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::ExecuteMsg,
    },
    util::validate::{Validate, ValidateResult},
};

use self::deposit_initial_drawdown::deposit_initial_drawdown;
use self::propose_commitment::propose_commitment;
use self::{accept_commitments::accept_commitments, withdraw_commitment::withdraw_commitment};

mod accept_commitments;
mod deposit_initial_drawdown;
mod propose_commitment;
mod withdraw_commitment;

pub fn run(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::ProposeCommitment { securities } => {
            propose_commitment(deps, info.sender, securities)
        }
        ExecuteMsg::AcceptCommitment { commitments } => accept_commitments(env, deps, commitments),
        ExecuteMsg::DepositInitialDrawdown { securities } => {
            deposit_initial_drawdown(deps, info.sender, info.funds, securities)
        }
        ExecuteMsg::WithdrawCommitment {} => withdraw_commitment(),
    }
}

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }
}
