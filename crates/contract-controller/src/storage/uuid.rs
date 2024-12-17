use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::{Item, Map};

use crate::core::{
    constants::{UUID_CACHE_KEY, UUID_KEY},
    error::ContractError,
};

// We store our securities that we configured on initialization
pub const UUID_MAP: Map<&str, Addr> = Map::new(UUID_KEY);
pub const UUID_CACHE: Item<String> = Item::new(UUID_CACHE_KEY);

pub fn has(storage: &dyn Storage, uuid: &str) -> bool {
    UUID_MAP.has(storage, uuid)
}

pub fn add(storage: &mut dyn Storage, uuid: &str, contract: &Addr) -> Result<(), ContractError> {
    Ok(UUID_MAP.save(storage, uuid, contract)?)
}

pub fn remove(storage: &mut dyn Storage, uuid: &str) {
    UUID_MAP.remove(storage, uuid);
}

pub fn get(storage: &dyn Storage, uuid: &str) -> Result<Addr, ContractError> {
    Ok(UUID_MAP.load(storage, uuid)?)
}

pub fn set_last_uuid(storage: &mut dyn Storage, uuid: &String) -> Result<(), ContractError> {
    Ok(UUID_CACHE.save(storage, uuid)?)
}

pub fn get_last_uuid(storage: &dyn Storage) -> Result<String, ContractError> {
    Ok(UUID_CACHE.load(storage)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::storage::uuid::{add, get, get_last_uuid, remove, set_last_uuid};

    use super::{has, UUID_MAP};

    #[test]
    fn test_has_success() {
        let mut deps = mock_provenance_dependencies();
        let contract = Addr::unchecked("addr");
        let uuid = "uuid";
        UUID_MAP
            .save(deps.as_mut().storage, &uuid, &contract)
            .unwrap();
        assert_eq!(true, has(&deps.storage, &uuid));
    }

    #[test]
    fn test_get_success() {
        let mut deps = mock_provenance_dependencies();
        let uuid = "uuid".to_string();
        set_last_uuid(deps.as_mut().storage, &uuid).unwrap();
        assert_eq!(uuid, get_last_uuid(&deps.storage).unwrap());
    }

    #[test]
    fn test_get_set() {
        let mut deps = mock_provenance_dependencies();
        let contract = Addr::unchecked("addr");
        let uuid = "uuid";
        UUID_MAP
            .save(deps.as_mut().storage, &uuid, &contract)
            .unwrap();
        assert_eq!(contract, get(&deps.storage, &uuid).unwrap());
    }

    #[test]
    fn test_has_failure() {
        let deps = mock_provenance_dependencies();
        let uuid = "uuid";
        assert_eq!(false, has(&deps.storage, &uuid));
    }

    #[test]
    fn test_add() {
        let mut deps = mock_provenance_dependencies();
        let uuid1 = "uuid1";
        let uuid2 = "uuid2";
        let contract1 = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &uuid1, &contract1).unwrap();
        add(deps.as_mut().storage, &uuid2, &contract2).unwrap();
        assert_eq!(true, has(&deps.storage, &uuid1));
        assert_eq!(true, has(&deps.storage, &uuid2));
    }

    #[test]
    fn test_remove() {
        let mut deps = mock_provenance_dependencies();
        let uuid1 = "uuid1";
        let uuid2 = "uuid2";
        let contract1 = Addr::unchecked("addr");
        let contract2 = Addr::unchecked("addr2");
        add(deps.as_mut().storage, &uuid1, &contract1).unwrap();
        add(deps.as_mut().storage, &uuid2, &contract2).unwrap();
        remove(deps.as_mut().storage, &uuid1);
        assert_eq!(false, has(&deps.storage, &uuid1));
        assert_eq!(true, has(&deps.storage, &uuid2));
    }

    #[test]
    fn test_remove_non_existant() {
        let mut deps = mock_provenance_dependencies();
        let uuid = "uuid";
        remove(deps.as_mut().storage, &uuid);
        assert!(true);
    }
}
