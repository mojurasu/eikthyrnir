use colored::Colorize;
use git2::Repository;
use regex::Regex;
use structopt::StructOpt;

use crate::error::Result;
use crate::utils::{padded_message, print_error};
use std::path::PathBuf;

/// Open anything
///
/// Useful on windows since there's nothing like xdg-open there
#[derive(StructOpt, Debug)]
pub struct  Open {
    /// Open the specified item or the current working directory
    item: Option<String>
}

pub fn open(opt: Open) -> Result<()> {
    let to_open = match opt.item {
        Some(i) => i,
        None => ".".to_string(),

    };
    padded_message("Opening".bright_blue().bold(), &to_open);
    open::that(to_open)?;
    Ok(())
}
