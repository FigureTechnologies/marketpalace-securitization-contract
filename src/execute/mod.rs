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
    settlement::withdraw_commitments,
};
pub mod settlement;

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

    fn validate_msg_funds(&self, funds: &Vec<cosmwasm_std::Coin>) -> ValidateResult {
        return match self {
            ExecuteMsg::DepositCommitment { securities: _ } => {
                if funds.is_empty() {
                    return Err(ContractError::MissingFunds {});
                }
                return Ok(());
            }
            _ => {
                if !funds.is_empty() {
                    return Err(ContractError::UnexpectedFunds {});
                }
                Ok(())
            }
        };
    }
}
