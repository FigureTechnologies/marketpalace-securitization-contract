use cli::Cli;

mod cli;
mod instantiate;
mod security;
mod user;

fn main() {
    let mut cli = Cli::new();
    cli.run();
    cli.handle_input();
}
