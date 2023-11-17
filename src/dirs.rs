use anyhow::{Context, Result};
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

fn get_dir(de: DirEntry) -> Option<PathBuf> {
    de.file_type()
        .ok()
        .and_then(|v| v.is_dir().then(|| de.path()))
}

pub fn get_dirs<P: AsRef<Path>>(p: P) -> Result<Vec<PathBuf>> {
    let path = p.as_ref().to_path_buf();
    let mut dirs = vec![];

    if path.is_dir() {
        if is_git_dir(&path) {
            dirs.push(path.clone());
        } else {
            let paths = path
                .read_dir()
                .with_context(|| format!("Reading directory {path:?}"))?;

            for path in paths {
                if let Some(path) = path.ok().and_then(get_dir) {
                    dirs.extend(get_dirs(path)?)
                }
            }
        }
    }

    Ok(dirs)
}

pub fn is_git_dir<P: AsRef<Path>>(p: P) -> bool {
    let mut path = p.as_ref().to_path_buf();
    path.push(".git");
    path.exists()
}
