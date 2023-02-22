use std::{
    io::{stdout, Write},
    str::FromStr,
};

use cosmwasm_std::Uint128;

pub fn get_input(text: &str) -> String {
    let mut line = String::new();
    print!("{}", text);
    stdout().flush().expect("should be able to flush");
    std::io::stdin().read_line(&mut line).unwrap();
    stdout().flush().expect("should be able to flush");
    line.trim().to_string()
}

pub fn get_int_input(text: &str) -> u128 {
    let input = get_input(text);
    Uint128::from_str(input.as_str())
        .expect("Should be able to parse input as u128.")
        .u128()
}
