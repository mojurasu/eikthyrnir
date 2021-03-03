use std::path::PathBuf;

use clap::Clap;

use error::Result;
use lib::duration::Duration;

mod error;
pub mod subcommands;
mod lib;

#[derive(Clap, Debug)]
#[clap(name = "basic")]
enum Opt {
    /// Relabel the worlds to their filename
    Delete {
        age: Duration,
        /// List of worlds to rename
        paths: Vec<PathBuf>,
    },
    /// Combined multiple worlds into one file
    List {
        age: Duration,
        /// A list of worlds that should be combined
        paths: Vec<PathBuf>,
    },
}


pub fn main() -> Result<()> {
    let opt: Opt = Opt::parse();
    match opt {
        Opt::Delete { paths, age } => subcommands::delete(paths, age)?,
        Opt::List { paths, age } => subcommands::list(paths, age)?,
    }
    Ok(())
}
