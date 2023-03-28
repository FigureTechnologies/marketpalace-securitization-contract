use contract::core::msg::QueryMsg;

pub fn create() {
    let message = QueryMsg::QueryCommitments {
        commitment_state: contract::execute::settlement::commitment::CommitmentState::PENDING,
    };
    let json = serde_json::to_string(&message).unwrap();
    println!("{}", json.trim());
}
