use structopt::StructOpt;

mod error;
mod modules;
mod utils;
use error::Result;
use crate::modules::git::Git;
use crate::modules::open::Open;

const PADDING: usize = 15;

#[derive(StructOpt, Debug)]
enum Opt {
    Git(Git),
    Open(Open)
}


fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Git(opt) => modules::git::git(opt),
        Opt::Open(opt) => modules::open::open(opt)
    }?;
    Ok(())
}
