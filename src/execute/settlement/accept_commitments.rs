use cosmwasm_std::{Addr, Response};

use crate::{
    commitment::CommitmentState,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
        state::{COMMITS, PAID_IN_CAPITAL, STATE},
    },
};

pub fn handle(deps: ProvDepsMut, sender: Addr, commitments: Vec<Addr>) -> ProvTxResponse {
    let state = STATE.load(deps.storage)?;
    if sender != state.gp {
        return Err(crate::core::error::ContractError::Unauthorized {});
    }

    for lp in commitments {
        let mut commitment = COMMITS.load(deps.storage, lp.clone())?;

        if commitment.state != CommitmentState::PENDING {
            // TODO Throw an error
            return Err(ContractError::InvalidCommitmentState {});
        }

        commitment.state = CommitmentState::ACCEPTED;
        COMMITS.save(deps.storage, lp.clone(), &commitment)?;

        // Create a new commit that can be updated when the LP deposits
        let mut commit = commitment.clone();
        commit.clear_amounts();
        PAID_IN_CAPITAL.save(deps.storage, lp.clone(), &commit.commitments)?;
    }
    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_handle_funds_are_empty() {
        assert!(false);
    }

    #[test]
    fn test_sender_must_be_gp() {
        assert!(false);
    }

    #[test]
    fn test_all_commitment_states_must_be_pending() {
        assert!(false);
    }

    #[test]
    fn test_paid_in_capital_is_set() {
        assert!(false);
    }

    #[test]
    fn test_all_states_are_updated_to_accepted() {
        assert!(false);
    }
}
