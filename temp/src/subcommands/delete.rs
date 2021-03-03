use std::path::PathBuf;

use eikthyrnir::*;

use crate::Result;
use crate::lib::duration::Duration;

pub fn delete(paths: Vec<PathBuf>, age: Duration) -> Result<()> {
    for path in paths {
        padded_message("Directory".bright_blue(), &path.to_string_lossy().to_string());
        let (succeeded, failed) = crate::lib::io::delete_files(crate::lib::io::list_files(path, &age)?)?;
        padded_message("Succeeded".bright_green(), succeeded.to_string());
        padded_message("Failed".bright_red(), failed.to_string());
        println!()
    };
    Ok(())
}
