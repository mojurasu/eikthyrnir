use std::process::Command;

use clap::{App, AppSettings};

use error::Result;

pub const PADDING: usize = 15;

mod utils;


fn main() -> () {
    let m = App::new("e")
        .setting(AppSettings::AllowExternalSubcommands)
        .get_matches();
    let subcommand = m.subcommand();
    let command_name = format!("{}-{}", env!("CARGO_PKG_NAME"), subcommand.0);
    let empty_args = vec![];

    let args = match subcommand.1 {
        Some(m) => {
            if !&m.args.is_empty() {
                &m.args[""].vals
            } else {
                &empty_args
            }
        }
        None => &empty_args
    };
    match Command::new(&command_name).args(args).spawn() {
        Ok(_) => (),
        Err(e) => utils::print_error(format!("running {}. Is it installed?", command_name))
    }
}
