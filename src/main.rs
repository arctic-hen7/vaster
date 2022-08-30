mod cli;
mod errors;

use clap::Parser;
use cli::{Opts, Subcommand};
use errors::Error;
use fmterr::fmterr;

static INFO: &str = include_str!("info.txt");

fn main() {
    let res = core();
    match res {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(err) => {
            // Print any errors to the console and terminate
            eprintln!("{}", fmterr(&err));
            std::process::exit(1);
        }
    }
}

fn core() -> Result<i32, Error> {
    let opts = Opts::parse();

    let exit_code = match opts.subcmd {
        Subcommand::Info => {
            println!("{}", INFO);
            0
        }
        Subcommand::Cache => {
            todo!()
        }
    };

    Ok(exit_code)
}
