use cli::Cli;

mod cli;
mod query;
mod security;
mod tx;
mod user;

fn main() {
    let mut cli = Cli::new();
    cli.run();
    cli.handle_input();
}
