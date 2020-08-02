use colored::Colorize;
use std::process;
use structopt::StructOpt;

use rustdsplit::{run, Cli};

fn main() {
    let args = Cli::from_args();

    env_logger::init();

    if let Err(e) = run(&args) {
        eprintln!("{} Error encountered: {}", "[!]".bright_red(), e);
        process::exit(1);
    }
}
