use cosmwasm_std::{to_json_binary, Storage, Uint128};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryStateResponse},
    storage,
};

pub fn handle(storage: &dyn Storage) -> ProvQueryResponse {
    let state = storage::state::get(storage)?;
    let response = QueryStateResponse {
        batch_size: Uint128::new(state.batch_size),
        migrating: state.migrating,
    };
    Ok(to_json_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, testing::mock_env, Uint128};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::msg::QueryStateResponse, query::query_state::handle,
        util::testing::instantiate_contract,
    };

    #[test]
    fn test_query_state_has_correct_default_values() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let _ = instantiate_contract(deps.as_mut(), env).unwrap();
        let bin_response = handle(&deps.storage).unwrap();
        let response: QueryStateResponse = from_json(&bin_response).unwrap();
        assert_eq!(Uint128::new(2), response.batch_size);
        assert_eq!(false, response.migrating);
    }
}
