use std::{path::{ Path, PathBuf }, fs::canonicalize};
pub fn expand(to_expand: &str) -> PathBuf {
    shellexpand::full(to_expand).unwrap().to_string().into()
}

pub fn expand_path<P:AsRef<Path>>(to_expand: P) -> PathBuf {
    let x = to_expand.as_ref().to_string_lossy();
    let x = expand(&x);
    canonicalize(x).unwrap()
}

