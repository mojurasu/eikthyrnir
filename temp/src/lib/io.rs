use std::path::PathBuf;
use crate::lib::duration::Duration;
use walkdir::WalkDir;
use crate::error::Result;
use std::fs;
use crate::lib::exclude::load_excluded;

pub fn list_files(path: PathBuf, age: &Duration) -> Result<Vec<PathBuf>> {
    let exclusions = load_excluded(&path)?;
    let mut files: Vec<PathBuf> = vec![];
    'outer: for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok())  {
        if !entry.path_is_symlink() {
            let modified = entry.metadata()?.created()?;
            if modified.elapsed().unwrap().as_secs() > age.seconds {
                let _path = entry.into_path();
                for p in &exclusions {
                    if _path.starts_with(p) {
                        continue 'outer
                    }
                }

                if _path != path {
                    files.push(_path)
                }
            }
        }
    }
    Ok(files)
}

pub fn delete_files(paths: Vec<PathBuf>) -> Result<(u64, u64)> {
    let mut succeeded = 0;
    let mut failed = 0;
    for path in paths {
        if path.is_file() {
            match fs::remove_file(path) {
                Ok(_) => succeeded += 1,
                Err(_) => failed += 1
            };
        } else if path.is_dir() {
            match fs::remove_dir_all(path) {
                Ok(_) => succeeded += 1,
                Err(_) => failed += 1
            };
        }
    };
    Ok((succeeded, failed))
}
