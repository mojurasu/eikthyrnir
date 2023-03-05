# eithyrnir
## Boilerplate:
```toml
# Cargo.toml
[dependencies]
eikthyrnir = {path = "../eikthyrnir"}
clap = { version = "4.1", features = ["derive"] }
```

```rust
// main.rs
use eikthyrnir::*;
use clap::Parser;

use error::Result;

#[derive(Parser, Debug)]
pub struct Args {}

pub fn main() -> Result<()> {
    let args = Args::parse();
    Ok(())
}
```
