use contract::core::msg::QueryMsg;
use cosmwasm_std::Addr;

pub fn create(investor: &str) {
    let message = QueryMsg::QueryInvestor {
        investor: Addr::unchecked(investor),
    };
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}
