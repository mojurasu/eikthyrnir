use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod subcommands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Args {
    /// Relabel the worlds to their filename
    Relabel {
        /// List of worlds to rename
        files: Vec<PathBuf>
    },
    /// Combined multiple worlds into one file
    Combine {
        /// The file the worls should be saved into
        file: PathBuf,
        /// A list of worlds that should be combined
        files: Vec<PathBuf>,
    },
}

pub fn main() -> Result<()> {
    let opt = Args::parse();
    match opt {
        Args::Relabel { files } => subcommands::relabel(files)?,
        Args::Combine { file, files } => subcommands::combine(file, files)?,
    }
    Ok(())
}
