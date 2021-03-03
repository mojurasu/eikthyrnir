use std::path::PathBuf;

use eikthyrnir::*;

use crate::Result;
use crate::Duration;

pub fn list(paths: Vec<PathBuf>, age: Duration) -> Result<()> {
    for path in paths {
        padded_message("Directory".bright_blue(), &path.to_string_lossy().to_string());
        for file in crate::lib::io::list_files(path, &age)? {
            padded_message("".white(), &file.to_string_lossy().to_string());
        }
    };
    Ok(())
}
