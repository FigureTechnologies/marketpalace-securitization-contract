use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::{Item};
use crate::core::error::ContractError;
use crate::core::security::LoanPoolContributors;


const KEY: &'static str = "whitelist_contributors";
pub const WHITELIST: Item<Vec<Addr>> = Item::new(KEY);

impl LoanPoolContributors {
    pub fn human_whitelist(&self) -> Vec<String> {
        self.address.iter().map(|a| a.to_string()).collect()
    }
}

pub fn save_contributors(
    storage: &mut dyn Storage,
    new_contributors: Vec<Addr>,
) -> Result<(), ContractError> {
    let contributors = WHITELIST.load(storage).unwrap_or_else(|_| vec![]);
    let mut updated_contributors = contributors.clone();
    updated_contributors.extend(new_contributors.iter().cloned());
    WHITELIST.save(storage, &updated_contributors)?;

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::{Addr, StdResult};
    #[test]
    fn test_save_contributors() -> StdResult<()> {
        let mut storage = MockStorage::new();
        let addr1 = Addr::unchecked("addr1");
        let addr2 = Addr::unchecked("addr2");

        // Test saving some contributors
        let contributors = vec![addr1.clone(), addr2.clone()];
        save_contributors(&mut storage, contributors.clone()).unwrap();
        let stored_contributors: Vec<Addr> = WHITELIST.load(&storage).unwrap();
        assert_eq!(stored_contributors, contributors);

        // Test saving an additional contributor, which should append to the existing ones
        let addr3 = Addr::unchecked("addr3");
        let additional_contributors = vec![addr3.clone()];
        save_contributors(&mut storage, additional_contributors.clone()).unwrap();
        let expected_contributors = vec![addr1.clone(), addr2.clone(), addr3.clone()];
        let stored_contributors: Vec<Addr> = WHITELIST.load(&storage).unwrap();
        assert_eq!(stored_contributors, expected_contributors);

        Ok(())
    }
}