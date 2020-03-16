# eithyrnir
## Boilerplate:
```toml
# Cargo.toml
[dependencies]
eikthyrnir = {path = "../eikthyrnir"}
structopt = "0.3"
```

```rust
// main.rs
use eikthyrnir::*;

mod error;
use error::Result;

#[derive(StructOpt, Debug)]
pub struct Opt {
}

pub fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    Ok(())
}
```

```rust
/// error.rs
use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Error,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for Error {
    fn from(item: io::Error) -> Self {
        Error::IO(item)
    }
}
```
