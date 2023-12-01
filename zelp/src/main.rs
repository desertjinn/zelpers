use std::borrow::BorrowMut;
use std::fmt::Debug;

use clap::Parser;

mod action;
mod logger;


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
    help: bool
}

fn main() {
    let args = Cli::parse();
    if args.debug {
        println!("WARNING: debug flag enabled");
        logger::set_debug_flag(Some(args.debug));
    }

    logger::debug("HAHAHAHA");

    // Print help manually
    // let mut cmd = Cli::command();
    // let _ = cmd.print_help();
}


