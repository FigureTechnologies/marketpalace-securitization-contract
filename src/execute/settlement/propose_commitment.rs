use cosmwasm_std::{Addr, Response};

use crate::{
    commitment::Commitment,
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        security::SecurityCommitment,
        state::COMMITS,
    },
};

pub fn handle(deps: ProvDepsMut, lp: Addr, commitments: Vec<SecurityCommitment>) -> ProvTxResponse {
    // TODO We probably want to validate the minimums
    // TODO We want to check to make sure there are no funds

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
