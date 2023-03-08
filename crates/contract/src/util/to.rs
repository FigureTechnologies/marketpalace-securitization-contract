use cosmwasm_std::Addr;

pub fn security_to_investment_name(name: &String, contract: &Addr) -> String {
    format! {"{}.{}", contract, name }
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
