use anyhow::{Context, Result};
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

fn get_dir(de: DirEntry) -> Option<PathBuf> {
    let dir_entry = de.file_type().expect("Getting file type");
    dir_entry.is_dir().then(|| de.path())
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
                if let Ok(path) = path {
                    if let Some(path) = get_dir(path) {
                        dirs.extend(get_dirs(path)?)
                    }
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
