use cosmwasm_std::{Addr, Coin, StdResult, Storage};
use cw_storage_plus::Map;

use crate::core::{constants::AVAILABLE_CAPITAL_KEY, error::ContractError};

pub const AVAILABLE_CAPITAL: Map<Addr, Vec<Coin>> = Map::new(AVAILABLE_CAPITAL_KEY);

pub fn add_capital(
    storage: &mut dyn Storage,
    lp: Addr,
    funds: Vec<Coin>,
) -> Result<(), ContractError> {
    AVAILABLE_CAPITAL.update(storage, lp, |available_capital| -> StdResult<Vec<Coin>> {
        match available_capital {
            None => Ok(funds),
            Some(mut available_capital) => {
                for fund_coin in &funds {
                    add_to_capital(fund_coin, &mut available_capital);
                }
                Ok(available_capital)
            }
        }
    })?;

    Ok(())
}

pub fn get_capital(storage: &mut dyn Storage, lp: Addr) -> Result<Vec<Coin>, ContractError> {
    Ok(AVAILABLE_CAPITAL.load(storage, lp)?)
}

pub fn get_lps(storage: &dyn Storage) -> Result<Vec<Addr>, ContractError> {
    let keys: StdResult<Vec<_>> = AVAILABLE_CAPITAL
        .keys(storage, None, None, cosmwasm_std::Order::Ascending)
        .collect();
    Ok(keys.unwrap())
}

pub fn remove_capital(storage: &mut dyn Storage, lp: Addr) -> Result<Coin, ContractError> {
    let capital = AVAILABLE_CAPITAL.load(storage, lp.clone())?;
    AVAILABLE_CAPITAL.remove(storage, lp);
    Ok(capital[0].clone())
}

pub fn has_lp(storage: &dyn Storage, lp: Addr) -> bool {
    AVAILABLE_CAPITAL.has(storage, lp)
}

// The purpose of this function is to add a coin to capital.
// We do this by finding the coin that has the same name as new_coin,
// and then we add the new_coin.amount to the coin.amount.
//
// Note this modifies capital
fn add_to_capital(new_coin: &Coin, capital: &mut [Coin]) {
    for coin in capital.iter_mut() {
        if coin.denom == new_coin.denom {
            coin.amount += new_coin.amount;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin, Uint128};
    use provwasm_mocks::mock_dependencies;

    use crate::storage::available_capital::{add_capital, add_to_capital, has_lp, remove_capital};

    use super::{get_capital, get_lps};

    #[test]
    fn test_add_to_capital_works_with_empty() {
        let denom = "denom".to_string();
        let coin = Coin::new(100, denom);
        let mut capital = vec![];
        add_to_capital(&coin, &mut capital);

        assert_eq!(0, capital.len());
    }

    #[test]
    fn test_add_to_capital_updates_first_capital() {
        let denom = "denom".to_string();
        let coin = Coin::new(100, denom.clone());
        let mut capital = vec![Coin::new(100, denom.clone()), Coin::new(100, denom.clone())];
        add_to_capital(&coin, &mut capital);

        assert_eq!(2, capital.len());
        assert_eq!(Coin::new(200, denom.clone()), capital[0]);
        assert_eq!(Coin::new(100, denom.clone()), capital[1]);
    }

    #[test]
    fn test_add_to_capital_ignores_invalid_coin() {
        let denom = "denom".to_string();
        let denom2 = "denom2".to_string();
        let coin = Coin::new(100, denom.clone());
        let mut capital = vec![
            Coin::new(100, denom2.clone()),
            Coin::new(100, denom.clone()),
        ];
        add_to_capital(&coin, &mut capital);

        assert_eq!(2, capital.len());
        assert_eq!(Coin::new(100, denom2.clone()), capital[0]);
        assert_eq!(Coin::new(200, denom.clone()), capital[1]);
    }

    #[test]
    fn test_remove_capital_success() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        let funds = vec![Coin::new(50, "denom".to_string())];
        add_capital(deps.as_mut().storage, lp.clone(), funds).unwrap();

        let removed = remove_capital(deps.as_mut().storage, lp.clone()).unwrap();
        assert_eq!(Uint128::new(50), removed.amount);
        assert_eq!(false, has_lp(deps.as_mut().storage, lp));
    }

    #[test]
    fn test_remove_capital_handles_invalid_lp() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        remove_capital(deps.as_mut().storage, lp).unwrap_err();
    }

    #[test]
    fn test_has_lp() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let funds = vec![Coin::new(50, "denom".to_string())];
        add_capital(deps.as_mut().storage, lp.clone(), funds).unwrap();

        assert!(has_lp(&deps.storage, lp));
        assert!(!has_lp(&deps.storage, Addr::unchecked("bad address")));
    }

    #[test]
    fn test_get_lps_not_empty() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let funds = vec![Coin::new(50, "denom".to_string())];
        add_capital(deps.as_mut().storage, lp.clone(), funds).unwrap();

        let lps = get_lps(&deps.storage).unwrap();
        assert_eq!(1, lps.len());
        assert_eq!(lp, lps[0]);

        let lp2 = Addr::unchecked("lp2");
        let funds2 = vec![Coin::new(50, "denom".to_string())];
        add_capital(deps.as_mut().storage, lp2.clone(), funds2).unwrap();
        let lps = get_lps(&deps.storage).unwrap();
        assert_eq!(2, lps.len());
        assert_eq!(lp, lps[0]);
        assert_eq!(lp2, lps[1]);
    }

    #[test]
    fn test_get_lps_empty() {
        let deps = mock_dependencies(&[]);

        let lps = get_lps(&deps.storage).unwrap();
        assert_eq!(0, lps.len());
    }

    #[test]
    fn test_get_capital_invalid() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("bad address");
        get_capital(deps.as_mut().storage, lp).unwrap_err();
    }

    #[test]
    fn test_get_capital_valid() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let funds = vec![Coin::new(50, "denom".to_string())];
        add_capital(deps.as_mut().storage, lp.clone(), funds.clone()).unwrap();

        let capital = get_capital(deps.as_mut().storage, lp).unwrap();
        assert_eq!(funds, capital);
    }

    #[test]
    fn test_add_capital_new_entry() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let funds = vec![Coin::new(50, "denom".to_string())];
        add_capital(deps.as_mut().storage, lp.clone(), funds.clone()).unwrap();

        let capital = get_capital(deps.as_mut().storage, lp).unwrap();
        assert_eq!(funds, capital);
    }

    #[test]
    fn test_add_capital_update_entry() {
        let mut deps = mock_dependencies(&[]);
        let lp = Addr::unchecked("lp");
        let funds = vec![Coin::new(50, "denom".to_string())];
        add_capital(deps.as_mut().storage, lp.clone(), funds.clone()).unwrap();
        add_capital(deps.as_mut().storage, lp.clone(), funds.clone()).unwrap();

        let capital = get_capital(deps.as_mut().storage, lp).unwrap();
        assert_eq!(vec![Coin::new(100, "denom".to_string())], capital);
    }
}
