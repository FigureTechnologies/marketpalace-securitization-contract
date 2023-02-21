use clap::{command, Arg, Command};
use contract;
use cosmwasm_std::Addr;

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
                    let message = contract::core::msg::InstantiateMsg {
                        gp: Addr::unchecked(gp),
                        securities: vec![],
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
