use contract::core::msg::ExecuteMsg;
use cosmwasm_std::Addr;

pub fn create(accepted: &Vec<String>) {
    let message = ExecuteMsg::AcceptCommitment {
        commitments: accepted
            .iter()
            .map(|address| Addr::unchecked(address))
            .collect(),
    };
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}
