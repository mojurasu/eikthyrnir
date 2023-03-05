use std::env;
use std::ffi::OsString;
use std::process::Command;

use anyhow::Result;

// #[cfg(target_os = "windows")]
// const ENV_PATH_SEPERATOR: char = ';';
// #[cfg(not(target_os = "windows"))]
// const ENV_PATH_SEPERATOR: char = ':';

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
            let mut available = Vec::new();


            if let Some(d) = env::current_exe()?.parent() {
                for e in d.read_dir()?.filter_map(|e| e.ok()) {
                    if e.file_type()?.is_file() {
                        if let Ok(filename) = e.file_name().into_string() {
                            #[cfg(target_os = "windows")]
                            if !filename.ends_with(".exe") {
                                continue;
                            }
                            if filename.starts_with(env!("CARGO_PKG_NAME")) {
                                #[cfg(target_os = "windows")]
                                    let filename = filename.strip_suffix(".exe").unwrap();
                                // TODO: Make this nice, can fail when executable has a hyphen
                                let cmd_name = filename.split("-").skip(1).next().unwrap().to_string();
                                available.push(cmd_name)
                            }
                        }
                    }
                }
            }

            if available.is_empty() {
                eikthyrnir::print_error("missing subcommand");
            } else {
                eikthyrnir::print_error(format!("missing subcommand. choose from the following: {}", available.join(", ")));
            }
            // TODO: Below is the technically correct implementation, however, it needs to filter out non-executables
            // for p in env::var("PATH")?.split(ENV_PATH_SEPERATOR).map(PathBuf::from_str).filter_map(|e| e.ok()) {
            //     // TODO: Need to expand ~ for ~/.cargo/bin
            //     if p.starts_with("~") {
            //         println!("{:?}", PathBuf::from(p.components().skip(1)));
            //     }
            //     if let Ok(r) = p.read_dir() {
            //         for e in r.filter_map(|e| e.ok()) {
            //             if e.file_type()?.is_file() {
            //                 if let Ok(filename) = e.file_name().into_string() {
            //                     if filename.starts_with(env!("CARGO_PKG_NAME")) && filename.ends_with("") {
            //                         // eikthyrnir::padded_message("".bright_blue(), filename)
            //                     }
            //                 }
            //
            //             }
            //
            //         }
            //
            //     }
            // }
        }
    };
    Ok(())
}
