use contract::core::msg::ExecuteMsg;
use cosmwasm_std::Addr;

pub fn create(accepted: &Vec<String>) -> ExecuteMsg {
    ExecuteMsg::AcceptCommitment {
        commitments: accepted
            .iter()
            .map(|address| Addr::unchecked(address))
            .collect(),
    }
}
