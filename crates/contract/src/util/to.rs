use cosmwasm_std::Addr;

/// Creates an investment token name for the supplied security name.
///
/// # Parameters
///
/// * `security_name` A reference to the name of the security.
/// * `contract` A reference to the address of the contract
///
/// # Examples
/// ```
/// use contract::util::to::security_to_investment_name;
///
/// let contract_address = Addr::unchecked("contract_address");
/// let security_name = "MySecurity".to_string();
/// let investment_name = security_to_investment_name(&security_name, &contract_address);
/// assert_eq!("contract_address.MySecurity".to_string(), investment_name);
///
/// ```
pub fn security_to_investment_name(security_name: &String, contract: &Addr) -> String {
    format! {"{}.{}", contract, security_name }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;

    use super::security_to_investment_name;

    #[test]
    fn test_security_to_investment_name() {
        let expected_name = String::from("contract_addr.security_a");
        let actual = security_to_investment_name(
            &"security_a".to_string(),
            &Addr::unchecked("contract_addr"),
        );
        assert_eq!(expected_name, actual);
    }
}
