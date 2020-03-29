use structopt::StructOpt;

mod error;
use error::Result;
use std::path::PathBuf;

mod subcommands;

#[derive(StructOpt, Debug)]
enum Opt {
    Relabel {
        files: Vec<PathBuf>
    }
}

pub fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    match opt {
        Opt::Relabel { files } => subcommands::relabel(files)?,
    }
    Ok(())
}
