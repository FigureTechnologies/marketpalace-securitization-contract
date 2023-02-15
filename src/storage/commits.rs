use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Map;

use crate::{
    core::{constants::COMMITS_KEY, error::ContractError},
    execute::settlement::commitment::Commitment,
};

pub const COMMITS: Map<Addr, Commitment> = Map::new(COMMITS_KEY);

pub fn get(storage: &dyn Storage, lp: Addr) -> Result<Commitment, ContractError> {
    Ok(COMMITS.load(storage, lp)?)
}

pub fn set(storage: &mut dyn Storage, commitment: &Commitment) -> Result<(), ContractError> {
    Ok(COMMITS.save(storage, commitment.lp.clone(), commitment)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use provwasm_mocks::mock_dependencies;

    use crate::{
        execute::settlement::commitment::Commitment, storage::commits::get, storage::commits::set,
    };

    #[test]
    fn test_get_invalid() {
        let deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        get(&deps.storage, lp).unwrap_err();
    }

    #[test]
    fn test_get_set_valid() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let commitment = Commitment::new(lp.clone(), vec![]);
        set(deps.as_mut().storage, &commitment).unwrap();

        let obtained = get(deps.as_mut().storage, lp).unwrap();
        assert_eq!(commitment, obtained);
    }
}
