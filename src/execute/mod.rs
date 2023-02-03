use cosmwasm_std::{Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
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
        match self {
            ExecuteMsg::ProposeCommitment { securities } => {
                if securities.len() == 0 {
                    return Err(ContractError::EmptySecurityCommitmentList {});
                }
                if securities.iter().any(|commitment| commitment.amount == 0) {
                    return Err(ContractError::InvalidSecurityCommitment {});
                }
            }
            ExecuteMsg::AcceptCommitment { commitments } => {
                if commitments.len() == 0 {
                    return Err(ContractError::EmptyAcceptedCommitmentList {});
                }
            }
            ExecuteMsg::DepositCommitment { securities } => {
                if securities.len() == 0 {
                    return Err(ContractError::EmptySecurityCommitmentList {});
                }
                if securities.iter().any(|commitment| commitment.amount == 0) {
                    return Err(ContractError::InvalidSecurityCommitment {});
                }
            }
            ExecuteMsg::WithdrawCommitments {} => {}
        };
        Ok(())
    }

    fn requires_funds(&self) -> bool {
        return match self {
            ExecuteMsg::DepositCommitment { securities: _ } => true,
            _ => false,
        };
    }
}
