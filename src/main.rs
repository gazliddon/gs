#![allow(dead_code)]
#![allow(unused_imports)]
mod config;
mod dirs;
mod expand;

use std::path::{Path, PathBuf};
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum StatusKind {
    ModifiedFiles,
    UntrackedFiles,
    BranchIsAhead,
    Clean,
}

struct Status {
    file: PathBuf,
    status: StatusKind,
}

impl Status {
    pub fn new<P: AsRef<Path>>(p: P, status: StatusKind) -> Self {
        Self {
            file: p.as_ref().into(),
            status,
        }
    }
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.file.file_stem().unwrap().to_string_lossy();
        write!(f, "{:20} {}", self.status.to_string(), file)
    }
}

impl std::fmt::Display for StatusKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusKind::ModifiedFiles => write!(f, "❌ Modified files"),
            StatusKind::UntrackedFiles => write!(f, "❌ Untracked files"),
            StatusKind::BranchIsAhead => write!(f, "❌ Branch is ahead"),
            StatusKind::Clean => write!(f, "✅ Clean"),
        }
    }
}

use regex::Regex;

fn check_for_string(r: &str, text: &str) -> bool {
    let regex_text = format!("({r})");
    let re = Regex::new(&regex_text).unwrap();
    re.is_match(text)
}

fn get_all_status<P: AsRef<Path>>(p: P) -> Vec<Status> {

    use std::env::{current_dir, set_current_dir};
    let p = p.as_ref();

    let mut ret = vec![];

    if dirs::is_git_dir(p) {
        use StatusKind::*;

        use std::process::Command;

        let dc = current_dir().expect("Can't get current_dir");
        set_current_dir(p).expect("Can't set current_dir");
        let out = Command::new("git").arg("status").output();
        set_current_dir(dc).expect("Can't set current_dir");

        match out {
            Ok(a) => {
                let text = std::str::from_utf8(&a.stdout).unwrap();
                if check_for_string("Untracked files:", text) {
                    ret.push(Status::new(p, UntrackedFiles))
                };

                if check_for_string("modified:", text) {
                    ret.push(Status::new(p, ModifiedFiles))
                };

                if check_for_string("Your branch is ahead", text) {
                    ret.push(Status::new(p, BranchIsAhead))
                };
                if ret.is_empty() {
                    ret.push(Status::new(&p, Clean))
                }
            }
            Err(_) => {
                panic!("Should not happen")
            }
        }
    } else {
        panic!("Interal Error!")
    }

    ret
}

fn main() {
    let opt = Opt::from_args();
    let config = config::Config::new();

    let mut files = opt.files;
    files.extend_from_slice(&config.repositries);

    println!("Files {:?}", files );

    for p in files {
        let x = dirs::get_dirs(p);

        let statues: Vec<_> = x.iter().map(get_all_status).flatten().collect();
        let mut txt: Vec<_> = statues.into_iter().map(|s| s.to_string()).collect();

        txt.sort();

        for t in txt {
            println!("{}", t)
        }
    }
}
