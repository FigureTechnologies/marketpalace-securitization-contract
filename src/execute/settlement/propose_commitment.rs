use cosmwasm_std::{Addr, Response};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    security::SecurityCommitment,
    state::{COMMITS, SECURITIES_MAP},
};

use super::commitment::Commitment;

pub fn handle(deps: ProvDepsMut, lp: Addr, commitments: Vec<SecurityCommitment>) -> ProvTxResponse {
    for commitment in &commitments {
        let security = SECURITIES_MAP.load(deps.storage, commitment.name.clone())?;
        if commitment.amount < security.minimum_amount {
            return Err(crate::core::error::ContractError::InvalidSecurityCommitmentAmount {});
        }
    }

    let commitment = Commitment::new(lp.clone(), commitments);

    COMMITS.save(deps.storage, lp, &commitment)?;
    Ok(Response::new())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_funds_are_empty() {
        assert!(false);
    }

    #[test]
    fn test_minimums_are_met() {
        assert!(false);
    }

    #[test]
    fn test_commit_is_added_on_success() {
        assert!(false);
    }
}
