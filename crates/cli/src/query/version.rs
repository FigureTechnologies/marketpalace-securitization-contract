use contract::core::msg::QueryMsg;

pub fn create() {
    let message = QueryMsg::QueryVersion {};
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}
