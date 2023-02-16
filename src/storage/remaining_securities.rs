use cosmwasm_std::{Storage, Uint128};
use cw_storage_plus::Map;

use crate::core::{constants::REMAINING_SECURITIES_KEY, error::ContractError, security::Security};

// We store our securities that we configured on initialization
pub const REMAINING_SECURITIES: Map<String, u128> = Map::new(REMAINING_SECURITIES_KEY);

pub fn get(storage: &dyn Storage, security_name: String) -> Result<u128, ContractError> {
    Ok(REMAINING_SECURITIES.load(storage, security_name)?)
}

pub fn set(
    storage: &mut dyn Storage,
    security_name: String,
    remaining: u128,
) -> Result<(), ContractError> {
    Ok(REMAINING_SECURITIES.save(storage, security_name, &remaining)?)
}

pub fn subtract(
    storage: &mut dyn Storage,
    security_name: String,
    amount: u128,
) -> Result<bool, ContractError> {
    let mut can_subtract = true;

    if !REMAINING_SECURITIES.has(storage, security_name.clone()) {
        return Ok(false);
    }

    let mut security_amount =
        Uint128::new(REMAINING_SECURITIES.load(storage, security_name.clone())?);

    match security_amount.checked_sub(Uint128::new(amount)) {
        Ok(new_value) => {
            security_amount = new_value;
            REMAINING_SECURITIES.save(storage, security_name, &security_amount.u128())?;
        }
        Err(_) => can_subtract = false,
    };

    Ok(can_subtract)
}

#[cfg(test)]
mod tests {
    use provwasm_mocks::mock_dependencies;

    use crate::storage::remaining_securities::{set, subtract};

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
        let security_name = "Security1".to_string();
        let amount = 100 as u128;
        set(deps.as_mut().storage, security_name.clone(), amount).unwrap();

        let obtained = get(deps.as_mut().storage, security_name).unwrap();
        assert_eq!(amount, obtained);
    }

    #[test]
    fn test_subtract_on_missing_entry() {
        let mut deps = mock_dependencies(&[]);
        let security_name = "Security1".to_string();
        let amount = 100 as u128;
        set(deps.as_mut().storage, security_name.clone(), amount).unwrap();

        let res = subtract(deps.as_mut().storage, security_name.clone(), 200).unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_subtract_on_greater_entry() {
        let mut deps = mock_dependencies(&[]);
        let security_name = "Security1".to_string();
        let amount = 100 as u128;
        set(deps.as_mut().storage, security_name.clone(), amount).unwrap();

        let res = subtract(deps.as_mut().storage, security_name.clone(), 200).unwrap();
        assert_eq!(false, res);
        let obtained = get(deps.as_mut().storage, security_name).unwrap();
        assert_eq!(amount, obtained);
    }

    #[test]
    fn test_subtract_success() {
        let mut deps = mock_dependencies(&[]);
        let security_name = "Security1".to_string();
        let amount = 100 as u128;
        set(deps.as_mut().storage, security_name.clone(), amount).unwrap();

        let res = subtract(deps.as_mut().storage, security_name.clone(), amount).unwrap();
        assert_eq!(true, res);
        let obtained = get(deps.as_mut().storage, security_name).unwrap();
        assert_eq!(0, obtained);
    }
}
