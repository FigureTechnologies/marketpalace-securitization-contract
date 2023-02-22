use contract::core::msg::ExecuteMsg;

pub fn create() {
    let message = ExecuteMsg::WithdrawCommitments {};
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}
