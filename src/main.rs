#![allow(dead_code)]
#![allow(unused_imports)]
mod dirs;
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
enum Status {
    ModifiedFiles,
    UntrackedFiles,
    BranchIsAhead,
    Clean,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::ModifiedFiles => write!(f, "❌ Modified files"),
            Status::UntrackedFiles => write!(f, "❌ Untracked files"),
            Status::BranchIsAhead => write!(f, "❌ Branch is ahead"),
            Status::Clean => write!(f, "✅ Clean"),
        }
    }
}

use regex::Regex;

fn get_it(r: &str, text: &str) -> bool {
    let regex_text = format!("({r})");
    let re = Regex::new(&regex_text).unwrap();
    re.is_match(text)
}

fn get_status<P: AsRef<Path>>(p: P) -> Vec<Status> {
    use std::env::{current_dir, set_current_dir};
    let p = p.as_ref();

    let mut ret = vec![];

    if dirs::is_git_dir(p) {
        use std::process::Command;

        let dc = current_dir().expect("Can't get current_dir");
        set_current_dir(p).expect("Can't set current_dir");
        let out = Command::new("git").arg("status").output();
        set_current_dir(dc).expect("Can't set current_dir");

        match out {
            Ok(a) => {
                let text = std::str::from_utf8(&a.stdout).unwrap();
                if get_it("Untracked files:", text) {
                    ret.push(Status::UntrackedFiles)
                };

                if get_it("modified:", text) {
                    ret.push(Status::ModifiedFiles)
                };

                if get_it("Your branch is ahead", text) {
                    ret.push(Status::BranchIsAhead)
                };
                if ret.is_empty() {
                    ret.push(Status::Clean)
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

    for p in opt.files {
        let x = dirs::get_dirs(p);

        let mut txt: Vec<_> = x
            .iter()
            .map(|d| {
                let dir = d.to_str().unwrap();
                let status = get_status(d);
                (dir, status)
            })
            .map(|(dir, status)| {
                status
                    .into_iter()
                    .filter(|s| *s != Status::Clean)
                    .map(|s| format!("{:18} : {}", s.to_string(), dir))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        txt.sort();

        for t in txt {
            println!("{}", t)
        }
    }
}
