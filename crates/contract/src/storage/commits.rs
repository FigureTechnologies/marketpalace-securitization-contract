use cosmwasm_std::{Addr, Order, Storage, Uint64};
use cw_storage_plus::Map;

use crate::{
    core::{constants::COMMITS_KEY, error::ContractError},
    execute::settlement::commitment::{Commitment, CommitmentState},
};

pub const COMMITS: Map<Addr, Commitment> = Map::new(COMMITS_KEY);

pub fn get(storage: &dyn Storage, lp: Addr) -> Result<Commitment, ContractError> {
    Ok(COMMITS.load(storage, lp)?)
}

pub fn set(storage: &mut dyn Storage, commitment: &Commitment) -> Result<(), ContractError> {
    Ok(COMMITS.save(storage, commitment.lp.clone(), commitment)?)
}

pub fn remove(storage: &mut dyn Storage, commitment_lp: Addr) {
    COMMITS.remove(storage, commitment_lp);
}

pub fn exists(storage: &dyn Storage, lp: Addr) -> bool {
    COMMITS.has(storage, lp)
}

pub fn get_with_state(storage: &dyn Storage, state: CommitmentState) -> Vec<Commitment> {
    let commits: Vec<Commitment> = COMMITS
        .range(storage, None, None, Order::Ascending)
        .filter(|item| item.is_ok() && item.as_ref().unwrap().1.state == state)
        .map(|item| item.unwrap().1)
        .collect();
    commits
}

pub fn set_settlement_time(
    storage: &mut dyn Storage,
    new_settlement_time: Option<Uint64>,
) -> Result<(), ContractError> {
    let commits: Vec<Commitment> = COMMITS
        .range(storage, None, None, Order::Ascending)
        .filter(Result::is_ok)
        .map(|item| item.unwrap().1)
        .collect();
    for mut commit in commits {
        commit.settlment_date = new_settlement_time;
        set(storage, &commit)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint64};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        execute::settlement::commitment::Commitment,
        storage::commits::{exists, get},
        storage::commits::{remove, set},
    };

    use super::set_settlement_time;

    #[test]
    fn test_get_invalid() {
        let deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("bad address");
        get(&deps.storage, lp).unwrap_err();
    }

    #[test]
    fn test_remove() {
        let mut deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp.clone(), vec![]);
        set(deps.as_mut().storage, &commitment).unwrap();
        remove(deps.as_mut().storage, lp.clone());
        assert_eq!(false, exists(deps.as_mut().storage, lp));
    }

    #[test]
    fn test_get_set_valid() {
        let mut deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp.clone(), vec![]);
        set(deps.as_mut().storage, &commitment).unwrap();

        let obtained = get(deps.as_mut().storage, lp).unwrap();
        assert_eq!(commitment, obtained);
    }

    #[test]
    fn test_exists() {
        let mut deps = mock_provenance_dependencies();
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp.clone(), vec![]);
        assert!(!exists(&deps.storage, lp.clone()));
        set(deps.as_mut().storage, &commitment).unwrap();
        assert!(exists(&deps.storage, lp));
    }

    #[test]
    fn test_update_settlement_time() {
        let mut deps = mock_provenance_dependencies();
        let lps = vec![Addr::unchecked("lp"), Addr::unchecked("lp2")];
        let settlement_time = Some(Uint64::new(9999));
        for lp in lps.clone() {
            let commitment = Commitment::new(lp.clone(), vec![]);
            set(deps.as_mut().storage, &commitment).unwrap();
        }
        set_settlement_time(deps.as_mut().storage, settlement_time).unwrap();
        for lp in lps {
            let commit = get(deps.as_ref().storage, lp).unwrap();
            assert_eq!(settlement_time, commit.settlment_date);
        }
    }

    #[test]
    fn test_update_settlement_time_empty() {
        let mut deps = mock_provenance_dependencies();
        set_settlement_time(deps.as_mut().storage, None).unwrap();
    }
}
