use structopt::StructOpt;

mod error;
mod modules;
mod utils;
use error::Result;
use crate::modules::git::Git;

const PADDING: usize = 15;

#[derive(StructOpt, Debug)]
enum Opt {
    Git(Git),
}


fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Git(opt) => modules::git::git(opt)
    }?;
    Ok(())
}
