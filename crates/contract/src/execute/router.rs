use cosmwasm_std::{Env, MessageInfo};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::ExecuteMsg,
};

use crate::execute::{
    settlement::propose_commitment,
    settlement::withdraw_commitment,
    settlement::{accept_commitments, deposit_commitment},
};

use super::settlement::{cancel_commitment, update_settlement_time, withdraw_all_commitments};

pub fn route(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::ProposeCommitment { securities } => {
            propose_commitment::handle(deps, &env, info.sender, securities)
        }
        ExecuteMsg::AcceptCommitment { commitments } => {
            accept_commitments::handle(deps, env, info.sender, commitments)
        }
        ExecuteMsg::DepositCommitment { securities } => {
            deposit_commitment::handle(deps, env, info.sender, info.funds, securities)
        }
        ExecuteMsg::WithdrawCommitment { lp } => {
            withdraw_commitment::handle(deps, env, info.sender, lp)
        }
        ExecuteMsg::WithdrawAllCommitments {} => {
            withdraw_all_commitments::handle(deps, env, info.sender)
        }
        ExecuteMsg::UpdateSettlementTime { settlement_time } => {
            update_settlement_time::handle(deps, info.sender, settlement_time)
        }
        ExecuteMsg::CancelCommitment { lp } => {
            cancel_commitment::handle(deps, env, info.sender, lp)
        }
        ExecuteMsg::ContributeLoanPool { loanPools } => {
            add_loanpool::handle(deps, env, info.sender, loanPools)
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_env;
    use provwasm_mocks::mock_dependencies;

    use crate::util::{self, testing::test_security_commitments};

    #[test]
    fn test_propose_commitment_is_routed() {
        let mut deps = mock_dependencies(&[]);
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp").unwrap();
    }

    #[test]
    fn test_accept_commitment() {
        let mut deps = mock_dependencies(&[]);
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp").unwrap();
        util::testing::accept_test_commitment(deps.as_mut(), mock_env(), "gp", &["lp"]).unwrap();
    }

    #[test]
    fn test_deposit_commitment() {
        let mut deps = mock_dependencies(&[]);
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp").unwrap();
        util::testing::accept_test_commitment(deps.as_mut(), mock_env(), "gp", &["lp"]).unwrap();
        util::testing::deposit_test(
            deps.as_mut(),
            mock_env(),
            "lp",
            &test_security_commitments(),
        )
        .unwrap();
    }

    #[test]
    fn test_withdraw_commitment() {
        let mut deps = mock_dependencies(&[]);
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp").unwrap();
        util::testing::accept_test_commitment(deps.as_mut(), mock_env(), "gp", &["lp"]).unwrap();
        util::testing::deposit_test(
            deps.as_mut(),
            mock_env(),
            "lp",
            &test_security_commitments(),
        )
        .unwrap();
        util::testing::withdraw_test(deps.as_mut(), mock_env(), "gp", "lp").unwrap();
    }

    #[test]
    fn test_withdraw_all_commitments() {
        let mut deps = mock_dependencies(&[]);
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp").unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp2").unwrap();
        util::testing::accept_test_commitment(deps.as_mut(), mock_env(), "gp", &["lp", "lp2"])
            .unwrap();
        util::testing::deposit_test(
            deps.as_mut(),
            mock_env(),
            "lp",
            &test_security_commitments(),
        )
        .unwrap();
        util::testing::withdraw_all_commitments_test(deps.as_mut(), mock_env(), "gp").unwrap();
    }

    #[test]
    fn test_update_settlement_time() {
        let mut deps = mock_dependencies(&[]);
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp").unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp2").unwrap();
        util::testing::accept_test_commitment(deps.as_mut(), mock_env(), "gp", &["lp", "lp2"])
            .unwrap();
        util::testing::deposit_test(
            deps.as_mut(),
            mock_env(),
            "lp",
            &test_security_commitments(),
        )
        .unwrap();
        util::testing::withdraw_test(deps.as_mut(), mock_env(), "gp", "lp").unwrap();
        util::testing::update_settlement_time_test(deps.as_mut(), mock_env(), "gp").unwrap();
    }

    #[test]
    fn test_cancel_commitment() {
        let mut deps = mock_dependencies(&[]);
        util::testing::instantiate_contract(deps.as_mut()).unwrap();
        util::testing::propose_test_commitment(deps.as_mut(), mock_env(), "lp").unwrap();
        util::testing::cancel_test(deps.as_mut(), mock_env(), "lp", "lp").unwrap();
    }
}
