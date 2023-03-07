use cosmwasm_std::{Addr, Env};

use crate::core::{aliases::ProvDepsMut, error::ContractError};

pub fn is_contract_admin(deps: &ProvDepsMut, env: &Env, addr: Addr) -> Result<bool, ContractError> {
    let admin = get_contract_admin(deps, env)?;
    Ok(admin == addr)
}

fn get_contract_admin(deps: &ProvDepsMut, env: &Env) -> Result<Addr, ContractError> {
    let response = deps
        .querier
        .query_wasm_contract_info(env.contract.address.clone())?;
    Ok(Addr::unchecked(response.admin.unwrap()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr};

    use crate::util::testing::{create_admin_deps, instantiate_contract};

    use super::{get_contract_admin, is_contract_admin};

    #[test]
    fn test_is_contract_admin_valid() {
        let env = mock_env();
        let mut deps = create_admin_deps(&[]);
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let admin = get_contract_admin(&deps.as_mut(), &env).unwrap();
        assert_eq!("admin", admin.to_string());
    }

    #[test]
    fn test_is_contract_admin_invalid() {
        let addr = Addr::unchecked("not admin");
        let env = mock_env();
        let mut deps = create_admin_deps(&[]);
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let is_admin = is_contract_admin(&deps.as_mut(), &env, addr).unwrap();
        assert_eq!(false, is_admin);
    }

    #[test]
    fn test_get_contract_admin_is_correct() {
        let addr = Addr::unchecked("admin");
        let env = mock_env();
        let mut deps = create_admin_deps(&[]);
        instantiate_contract(deps.as_mut(), env.clone()).unwrap();
        let is_admin = is_contract_admin(&deps.as_mut(), &env, addr).unwrap();
        assert_eq!(true, is_admin);
    }
}
