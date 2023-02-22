use std::{
    io::{stdout, Write},
    str::FromStr,
};

use clap::{command, Arg, Command};
use contract::{
    self,
    core::security::{FundSecurity, PrimarySecurity, Security, SecurityType, TrancheSecurity},
};
use cosmwasm_std::{Addr, Coin, Uint128};

fn get_input(text: &str) -> String {
    let mut line = String::new();
    print!("{}", text);
    stdout().flush().expect("should be able to flush");
    std::io::stdin().read_line(&mut line).unwrap();
    stdout().flush().expect("should be able to flush");
    line.trim().to_string()
}

fn get_int_input(text: &str) -> u128 {
    let input = get_input(text);
    Uint128::from_str(input.as_str())
        .expect("Should be able to parse input as u128.")
        .u128()
}

fn collect_securities(capital_denom: &String) -> Vec<Security> {
    let mut securities = vec![];

    let security_count = get_int_input("Enter number of securities: ");

    for _ in 0..security_count {
        let name = get_input("Enter security name: ");
        let security_type =
            match get_input("Enter security type (0 Tranche, 1 Primary, 2 Fund): ").as_str() {
                "0" => SecurityType::Tranche(TrancheSecurity {}),
                "1" => SecurityType::Primary(PrimarySecurity {}),
                "2" => SecurityType::Fund(FundSecurity {}),
                _ => panic!("Unexpected security type"),
            };
        let amount = get_int_input("Enter amount of security: ");
        let minimum_amount = get_int_input("Enter minimum amount of security: ");
        let price = get_int_input("Enter price per unit of security: ");
        let price_per_unit = Coin::new(price, capital_denom);

        securities.push(Security {
            name,
            amount,
            security_type,
            minimum_amount,
            price_per_unit,
        });
    }

    securities
}

fn main() {
    let mut cli = command!()
        .next_line_help(false)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("query")
                .alias("q")
                .arg_required_else_help(true)
                .about("Generate json for a query belonging to this smart contract."),
        )
        .subcommand(
            Command::new("transaction")
                .alias("tx")
                .arg_required_else_help(true)
                .about("Generate json for a transaction belonging to this smart contract.")
                .subcommand(
                    Command::new("initialize")
                        .alias("init")
                        .about("Create an initialize transaction.")
                        .arg(
                            Arg::new("gp")
                                .short('g')
                                .long("gp")
                                .required(true)
                                .help("The address of the gp"),
                        )
                        .arg(
                            Arg::new("capital_denom")
                                .short('c')
                                .long("capital_denom")
                                .required(true)
                                .help("The denomination to use for capital"),
                        ),
                )
                .subcommand(
                    Command::new("propose_commitment")
                        .alias("propose")
                        .about("Create a propose commitment transaction."),
                )
                .subcommand(
                    Command::new("accept_commitments")
                        .alias("accept")
                        .about("Create an initialize transaction")
                        .arg(
                            Arg::new("commits")
                                .short('c')
                                .long("commits")
                                .required(true)
                                .help("The addresses of one or more commits"),
                        ),
                )
                .subcommand(
                    Command::new("deposit_commitment")
                        .alias("deposit")
                        .about("Create a deposit commitment transaction."),
                )
                .subcommand(
                    Command::new("withdraw_commitments")
                        .alias("withdraw")
                        .about("Create a transaction to withdraw deposited funds"),
                ),
        );

    let matches = cli.get_matches();
    match matches.subcommand() {
        Some(("query", query_matches)) => match query_matches.subcommand() {
            _ => println!("Unrecognized query"),
        },
        Some(("transaction", tx_matches)) => {
            match tx_matches.subcommand() {
                Some(("initialize", init_matches)) => {
                    let gp: String = init_matches.get_one::<String>("gp").unwrap().clone();
                    let denom: String = init_matches
                        .get_one::<String>("capital_denom")
                        .unwrap()
                        .clone();
                    let securities = collect_securities(&denom);
                    let message = contract::core::msg::InstantiateMsg {
                        gp: Addr::unchecked(gp),
                        securities: securities,
                        capital_denom: denom,
                        rules: vec![],
                    };
                    let json = serde_json::to_string(&message).unwrap();
                    println!("{}", json.trim());
                }
                Some(("propose_commitment", _init_matches)) => {
                    println!("Running propose");
                }
                Some(("accept_commitments", _init_matches)) => {
                    println!("Running accept");
                }
                Some(("deposit_commitment", _init_matches)) => {
                    println!("Running deposit");
                }
                Some(("withdraw_commitments", _init_matches)) => {
                    println!("Running withdraw");
                }
                _ => println!("Unrecognized transaction"),
            };
        }
        _ => println!("Unrecognized command"),
    };
    // In here this is where we will start asking them to input securities and investment rules
}
