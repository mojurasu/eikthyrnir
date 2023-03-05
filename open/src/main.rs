use clap::Parser;
use eikthyrnir::*;
use anyhow::Result;

/// Open anything
///
/// Useful on windows since there's nothing like xdg-open there
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Open the specified item or the current working directory
    item: Option<String>
}

pub fn main() -> Result<()> {
    let opt = Args::parse();
    let to_open = match opt.item {
        Some(i) => i,
        None => ".".to_string(),
    };
    padded_message("Opening".bright_blue().bold(), &to_open);
    open::that_in_background(to_open);
    Ok(())
}
