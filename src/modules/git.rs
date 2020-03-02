use git2::Repository;
use regex::Regex;
use structopt::StructOpt;

use crate::error::Result;
use colored::Colorize;
use crate::utils::{print_error, padded_message};
use std::intrinsics::prefetch_read_instruction;

/// Helpers for git repositories
#[derive(StructOpt, Debug)]
pub enum Git {
    /// Open the assumed website from the origin
    Web,
}

// TODO: Support specifying origin
// TODO: Open with the current branch (can be disabled with a flag)
fn web() -> Result<()> {
    let repo = match Repository::open(".") {
        Ok(r) => r,
        Err(_) => {
            print_error("Not in a valid git repository");
            return Ok(())
        }
    };
    let remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => {
            print_error("remote `origin` does not exist");
            return Ok(())
        }
    };
    // If your remote url isnt valid UTF-8 you're doing something wrong
    let url = match remote.url() {
        Some(url) => url,
        None => {
            print_error("remote url not valid UTF-8");
            return Ok(())
        }
    };
    let protocol = Regex::new(r#"(https?://|git@|\.git$)"#)?;
    let url = protocol.replace(url, "").replace(":", "/");
    let web_url = format!("http://{}", url);
    padded_message("Opening".bright_purple().bold(), web_url);
    open::that(&web_url)?;
    Ok(())
}

pub fn git(opt: Git) -> Result<()> {
    match opt {
        Git::Web => web()
    }?;
    Ok(())
}
