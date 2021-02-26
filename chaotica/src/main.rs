use structopt::StructOpt;

mod error;
use error::Result;
use std::path::PathBuf;

mod subcommands;

#[derive(StructOpt, Debug)]
enum Opt {
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
    }
}

pub fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    match opt {
        Opt::Relabel { files } => subcommands::relabel(files)?,
        Opt::Combine { file, files } => subcommands::combine(file, files)?,
    }
    Ok(())
}
