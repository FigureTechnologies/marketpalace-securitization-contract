use contract::core::msg::ExecuteMsg;
use cosmwasm_std::Addr;

pub fn create(lp: String) {
    let message = ExecuteMsg::WithdrawCommitment {
        lp: Addr::unchecked(lp),
    };
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}
