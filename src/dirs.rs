use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::io;

fn get_dir(de: DirEntry) -> Result<PathBuf, io::Error> {
    if de.file_type().unwrap().is_dir() {
        Ok(de.path())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Oh no!"))
    }
}

pub fn get_dirs<P: AsRef<Path>>(p: P) -> Vec<PathBuf> {
    let path = p.as_ref().to_path_buf();
    let mut dirs = vec![];

    if path.is_dir() {
        if is_git_dir(&path) {
            dirs.push(path.clone());
        } else {
            for p in path.read_dir().expect("Can't read it!") {
                if let Ok(dir) = p.and_then(get_dir) {
                    dirs.extend(get_dirs(dir))
                }
            }
        }
    }

    dirs
}

pub fn is_git_dir<P: AsRef<Path>>(p: P) -> bool {
    let mut path = p.as_ref().to_path_buf();
    path.push(".git");
    path.exists()
}
