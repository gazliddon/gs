use std::path::{ Path, PathBuf };
pub fn expand(to_expand: &str) -> PathBuf {
    shellexpand::full(to_expand).unwrap().to_string().into()
}

pub fn expand_path<P:AsRef<Path>>(to_expand: P) -> PathBuf {
    let x = to_expand.as_ref().to_string_lossy();
    expand(&x)
}

