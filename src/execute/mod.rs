use cosmwasm_std::{Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::ExecuteMsg,
    },
    util::validate::{Validate, ValidateResult},
};

use self::{
    settlement::accept_commitments, settlement::deposit_commitment, settlement::propose_commitment,
};
mod settlement;
mod withdraw_commitments;

pub fn handle(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::ProposeCommitment { securities } => {
            propose_commitment::handle(deps, info.sender, securities)
        }
        ExecuteMsg::AcceptCommitment { commitments } => {
            accept_commitments::handle(deps, info.sender, commitments)
        }
        ExecuteMsg::DepositCommitment { securities } => {
            deposit_commitment::handle(deps, info.sender, info.funds, securities)
        }
        ExecuteMsg::WithdrawCommitments {} => withdraw_commitments::handle(deps, env, info.sender),
    }
}

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }
}
