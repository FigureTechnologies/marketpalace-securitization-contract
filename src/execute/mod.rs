use cosmwasm_std::{Coin, Env, MessageInfo};

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
pub mod validate;

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_propose_commitment() {
        assert!(false);
    }

    #[test]
    fn test_accept_commitment() {
        assert!(false);
    }

    #[test]
    fn test_deposit_commitment() {
        assert!(false);
    }

    #[test]
    fn test_withdraw_commitment() {
        assert!(false);
    }
}
