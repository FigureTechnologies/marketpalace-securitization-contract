use cosmwasm_std::{from_binary, testing::mock_env, to_binary, Storage};
use provwasm_mocks::mock_dependencies;

use crate::{
    contract::query,
    core::{
        aliases::ProvQueryResponse,
        msg::{QueryMsg, QueryStateResponse},
    },
    storage,
    util::testing::{instantiate_contract, test_init_message},
};

pub fn query_state(storage: &dyn Storage) -> ProvQueryResponse {
    let state = storage::state::get(storage)?;
    let securities = storage::securities::get_security_types(storage);
    let response = QueryStateResponse {
        gp: state.gp,
        securities,
        capital_denom: state.capital_denom,
        rules: state.rules,
    };
    Ok(to_binary(&response)?)
}

#[test]
fn test_has_correct_state() {
    let mut deps = mock_dependencies(&[]);
    instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
    let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryState {}).unwrap();
    let value: QueryStateResponse = from_binary(&res).unwrap();
    let expected = test_init_message();

    let securities: Vec<String> = expected
        .securities
        .iter()
        .map(|security| security.name.clone())
        .collect();

    assert_eq!(expected.gp, value.gp);
    assert_eq!(expected.capital_denom, value.capital_denom);
    assert_eq!(expected.rules, value.rules);
    assert_eq!(securities, value.securities);
}
