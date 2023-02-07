use cosmwasm_std::{Addr, Response, Storage};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    error::ContractError,
    state::{COMMITS, PAID_IN_CAPITAL, STATE},
};

use super::commitment::{Commitment, CommitmentState};

pub fn handle(deps: ProvDepsMut, sender: Addr, commitments: Vec<Addr>) -> ProvTxResponse {
    let state = STATE.load(deps.storage)?;
    if sender != state.gp {
        return Err(crate::core::error::ContractError::Unauthorized {});
    }

    for lp in commitments {
        let mut commitment = COMMITS.load(deps.storage, lp.clone())?;

        if commitment.state != CommitmentState::PENDING {
            return Err(ContractError::InvalidCommitmentState {});
        }

        commitment.state = CommitmentState::ACCEPTED;
        COMMITS.save(deps.storage, lp.clone(), &commitment)?;

        track_paid_capital(deps.storage, commitment)?;
    }
    Ok(Response::new())
}

fn track_paid_capital(
    storage: &mut dyn Storage,
    mut commitment: Commitment,
) -> Result<(), ContractError> {
    commitment.clear_amounts();
    PAID_IN_CAPITAL.save(storage, commitment.lp, &commitment.commitments)?;
    Ok(())
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
