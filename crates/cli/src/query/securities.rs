use contract::core::msg::QueryMsg;

pub fn create(securities: Vec<String>) {
    let message = QueryMsg::QuerySecuritizations { securities };
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}
