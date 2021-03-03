use crate::error::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn load_excluded(root: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut excluded = vec![];
    let exclude_file = root.join(".etempignore");
    if exclude_file.is_file() {
        let file = File::open(&exclude_file)?;
        let buf_reader = BufReader::new(file);
        for line in buf_reader.lines() {
            if let Ok(path) = line {
                excluded.push(root.join(path))
            };
        }
        excluded.push(exclude_file);
    }
    Ok(excluded)
}
