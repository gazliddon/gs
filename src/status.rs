use std::collections::{ HashMap, HashSet };
use std::path::{Path, PathBuf};
use crate::dirs;

use regex::Regex;

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd,Ord, Hash)]
pub enum StatusKind {
    ModifiedFiles,
    UntrackedFiles,
    BranchIsAhead,
    Clean,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Status {
    pub file: PathBuf,
    pub status: StatusKind,
}

impl Status {
    pub fn new<P: AsRef<Path>>(p: P, status: StatusKind) -> Self {
        Self {
            file: p.as_ref().into(),
            status,
        }
    }

    pub fn is_clean(&self) -> bool {
        self.status == StatusKind::Clean
    }
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _file = self.file.file_stem().map(|s| s.to_string_lossy()).unwrap();
        write!(
            f,
            "{:20} : {}",
            _file,
            self.status.to_string(),
        )
    }
}

impl std::fmt::Display for StatusKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusKind::ModifiedFiles => write!(f, "Modified files"),
            StatusKind::UntrackedFiles => write!(f, "Untracked files"),
            StatusKind::BranchIsAhead => write!(f, "Branch is ahead"),
            StatusKind::Clean => write!(f, "Clean"),
        }
    }
}

fn check_for_string(r: &str, text: &str) -> bool {
    let regex_text = format!("({r})");
    let re = Regex::new(&regex_text).unwrap();
    re.is_match(text)
}

#[derive(Debug, Default)]
pub struct StatusGetter {
    cache: HashMap<PathBuf, Status>,
    processed_dirs: HashSet<PathBuf>,
}

impl StatusGetter {
    pub fn new(paths: &[PathBuf]) -> Self {
        let mut ret = Self::default();

        for p in paths {
            ret.get_status(p)
        }

        ret
    }

    pub fn add_dir<P: AsRef<Path>>(&self, _dir: P ) {
    }

    fn get_git_dir_status<P: AsRef<Path>>(&self, dir: P) -> Vec<Status> {
        use std::env::{current_dir, set_current_dir};
        use std::process::Command;
        use StatusKind::*;

        let dir = dir.as_ref();

        let mut ret = vec![];

        let dc = current_dir().expect("Can't get current_dir");
        set_current_dir(dir).expect("Can't set current_dir");
        let out = Command::new("git").arg("status").output();
        set_current_dir(dc).expect("Can't set current_dir");

        match out {
            Ok(a) => {
                let text = std::str::from_utf8(&a.stdout).unwrap();
                if check_for_string("Untracked files:", text) {
                    ret.push(Status::new(dir, UntrackedFiles))
                };

                if check_for_string("modified:", text) {
                    ret.push(Status::new(dir, ModifiedFiles))
                };

                if check_for_string("Your branch is ahead", text) {
                    ret.push(Status::new(dir, BranchIsAhead))
                };
                if ret.is_empty() {
                    ret.push(Status::new(dir, Clean))
                }
            }
            Err(_) => {
                panic!("Should not happen")
            }
        }
        ret
    }

    fn get_status<P: AsRef<Path>>(&mut self, dir: P) {
        let dir = dir.as_ref();

        if !dirs::is_git_dir(dir) {
            panic!("Internal error")
        }

        if !self.cache.contains_key(dir) {
            let stats = self.get_git_dir_status(dir);

            for stat in stats {
                self.cache.insert(stat.file.clone(), stat);
            }
        }
    }

    pub fn to_statues(self) -> Vec<Status> {
        self.cache.values().cloned().collect()

    }
}
