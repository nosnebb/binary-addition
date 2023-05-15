use clap::{Command};

pub fn cli_command() -> () {
    let matches = Command::new("binary addition")
        .about("a utility to add binary numbers together")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("nosnebb");
        // query Command
        //
        // only a few arguments for this query command lul.
}
