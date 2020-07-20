use rustdsplit::{run, Cli};
use std::process;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    env_logger::init();

    if let Err(e) = run(&args) {
        eprintln!("[!] Error encountered: {}", e);
        process::exit(1);
    }
}
