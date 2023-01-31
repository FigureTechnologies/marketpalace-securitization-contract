use cosmwasm_std::Addr;

pub fn security_to_investment_name(name: &String, contract: &Addr) -> String {
    format! {"{}.{}.investment", contract, name }
}
