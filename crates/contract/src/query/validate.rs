impl Validate for QueryMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_msg_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}
