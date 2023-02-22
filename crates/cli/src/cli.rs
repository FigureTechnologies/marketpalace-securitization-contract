use clap::{command, value_parser, Arg, ArgAction, ArgMatches, Command};

use crate::tx;

pub struct Cli {
    cli: Command,
    args: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            cli: command!()
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
                                        .action(ArgAction::Append)
                                        .value_parser(value_parser!(String))
                                        .short('c')
                                        .long("commits")
                                        .required(true)
                                        .value_delimiter(',')
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
                ),
            args: ArgMatches::default(),
        }
    }

    pub fn run(&mut self) {
        self.args = self.cli.clone().get_matches();
    }

    pub fn handle_input(&mut self) {
        match self.args.subcommand() {
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
                        tx::instantiate::create(&gp, denom);
                    }
                    Some(("propose_commitment", _propose_matches)) => {
                        tx::propose_commitment::create();
                    }
                    Some(("accept_commitments", accept_matches)) => {
                        let accepted = accept_matches
                            .get_many::<String>("commits")
                            .unwrap()
                            .map(|value| value.clone())
                            .collect::<Vec<_>>();
                        tx::accept_commitments::create(&accepted);
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
    }
}
