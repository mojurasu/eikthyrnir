use std::env;
use std::ffi::OsString;
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    env::set_var("CLICOLOR_FORCE", "1");
    let m = clap::Command::new("e")
        .allow_external_subcommands(true)
        .get_matches();

    match m.subcommand() {
        Some((subcmd, ext_m)) => {
            let subcmd_args: Vec<_> = ext_m.get_many::<OsString>("").unwrap().collect();
            let executable = format!("{}-{}", env!("CARGO_PKG_NAME"), subcmd);
            match Command::new(&executable).args(subcmd_args).spawn() {
                Ok(mut c) => {
                    c.wait().expect("Failed while waiting for the child.");
                }
                Err(_) => eikthyrnir::print_error(format!("running {}. Is it installed?", executable))
            }
        }
        _ => {
            eikthyrnir::print_error("missing subcommand")
        }
    };
    Ok(())
}
