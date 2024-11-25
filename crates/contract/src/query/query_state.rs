use cosmwasm_std::{to_json_binary, Storage};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryStateResponse},
    storage,
};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let state = storage::state::get(storage)?;
    let securities = storage::securities::get_security_types(storage);
    let response = QueryStateResponse {
        gp: state.gp,
        securities,
        capital_denom: state.capital_denom,
        settlement_time: state.settlement_time,
    };
    Ok(to_json_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, testing::mock_env};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        contract::query,
        core::msg::{QueryMsg, QueryStateResponse},
        util::testing::{instantiate_contract, test_init_message},
    };

    #[test]
    fn test_has_correct_state() {
        let mut deps = mock_dependencies(&[]);
        instantiate_contract(deps.as_mut()).expect("should be able to instantiate contract");
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryState {}).unwrap();
        let value: QueryStateResponse = from_json(&res).unwrap();
        let expected = test_init_message();

        let securities: Vec<String> = expected
            .securities
            .iter()
            .map(|security| security.name.clone())
            .collect();

        assert_eq!(expected.gp, value.gp);
        assert_eq!(expected.capital_denom, value.capital_denom);
        assert_eq!(expected.settlement_time, value.settlement_time);
        assert_eq!(securities, value.securities);
    }
}
