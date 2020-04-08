use structopt::StructOpt;

use eikthyrnir::*;
use error::Result;

mod error;

/// Open anything
///
/// Useful on windows since there's nothing like xdg-open there
#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Open the specified item or the current working directory
    item: Option<String>
}

pub fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    let to_open = match opt.item {
        Some(i) => i,
        None => ".".to_string(),
    };
    padded_message("Opening".bright_blue().bold(), &to_open);
    open::that(to_open)?;
    Ok(())
}
