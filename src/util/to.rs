use cosmwasm_std::Addr;

pub fn security_to_investment_name(name: &String, contract: &Addr) -> String {
    format! {"{}.{}.investment", contract, name }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_security_to_investment_name() {
        assert!(false);
    }
}
