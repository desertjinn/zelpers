use std::borrow::BorrowMut;
use std::fmt::Debug;

use clap::Parser;
use traits::tty::Tty;

mod action;
mod log;
mod traits;
mod enums;
mod datastore;

#[derive(Parser,Default,Debug)]
#[clap(author="desertjinn", version="0.1.0", about="A zen helper script")]
#[command(arg_required_else_help = true, disable_help_flag = true)]
/// A useful CLI helper
/// usage: what?
struct Cli {
    /// The command you want to run
    command: String,
    /// The action to be performed by that command
    action: Option<String>,
    /// Enable debug messages
    #[clap(default_value("false"), short('d'), long("debug"), help("Enable the debug flag"))]
    debug: bool,
    #[clap(default_value("false"), short('h'), long("help"), help("Print help for Zelp"))]
    help: bool,
    #[clap(default_value(""), short('l'), long("log"), help("Set the log level"))]
    log: String,

}

fn main() {
    let logger = log::Logger{ data_store: None };
    let args = Cli::parse();
    println!("{:?}", args);

    if args.log != "" {
        logger.set_level(args.log);
    }

    logger.print("HEY!")

    // Print help manually
    // let mut cmd = Cli::command();
    // let _ = cmd.print_help();
}


