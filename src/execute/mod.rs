use cosmwasm_std::{Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::ExecuteMsg,
    },
    util::validate::{Validate, ValidateResult},
};

use self::{
    settlement::accept_commitments, settlement::deposit_initial_drawdown,
    settlement::propose_commitment,
};
mod settlement;
mod withdraw_capital;

pub fn handle(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::ProposeCommitment { securities } => {
            propose_commitment::handle(deps, info.sender, securities)
        }
        ExecuteMsg::AcceptCommitment { commitments } => {
            accept_commitments::handle(deps, info.sender, commitments)
        }
        ExecuteMsg::DepositInitialDrawdown { securities } => {
            deposit_initial_drawdown::handle(deps, info.sender, info.funds, securities)
        }
        ExecuteMsg::WithdrawCapital {} => withdraw_capital::handle(deps, env, info.sender),
    }
}

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }
}
