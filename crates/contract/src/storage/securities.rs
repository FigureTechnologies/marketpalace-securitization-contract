use cosmwasm_std::{Order, StdResult, Storage};
use cw_storage_plus::Map;

use crate::core::{constants::SECURITIES_MAP_KEY, error::ContractError, security::Security};

// We store our securities that we configured on initialization
pub const SECURITIES_MAP: Map<String, Security> = Map::new(SECURITIES_MAP_KEY);

pub fn get_security_types(storage: &dyn Storage) -> Vec<String> {
    let security_types: StdResult<Vec<_>> = SECURITIES_MAP
        .keys(storage, None, None, Order::Ascending)
        .collect();
    security_types.unwrap()
}

pub fn get(storage: &dyn Storage, security_name: String) -> Result<Security, ContractError> {
    Ok(SECURITIES_MAP.load(storage, security_name)?)
}

pub fn set(storage: &mut dyn Storage, security: &Security) -> Result<(), ContractError> {
    Ok(SECURITIES_MAP.save(storage, security.name.clone(), security)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Coin, Uint128};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::security::{Security, SecurityType, TrancheSecurity},
        storage::securities::set,
    };

    use super::get;

    #[test]
    fn test_get_invalid() {
        let deps = mock_dependencies(&[]);
        let security_name = "badname".to_string();
        get(&deps.storage, security_name).unwrap_err();
    }

    #[test]
    fn test_get_set_valid() {
        let mut deps = mock_dependencies(&[]);
        let security = Security {
            name: "Security1".to_string(),
            amount: Uint128::new(100),
            security_type: SecurityType::Tranche(TrancheSecurity {}),
            minimum_amount: Uint128::new(10),
            price_per_unit: Coin::new(100, "denom".to_string()),
        };
        set(deps.as_mut().storage, &security).unwrap();

        let obtained = get(deps.as_mut().storage, security.name.clone()).unwrap();
        assert_eq!(security, obtained);
    }
}
