#![allow(dead_code)]
#![deny(unused_imports)]
mod config;
mod dirs;
mod expand;
mod status;

use itertools::Itertools;

use std::{intrinsics::unlikely, path::PathBuf};
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

    #[structopt(short, long)]
    noclean: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_str = expand::expand_path))]
    dirs: Vec<PathBuf>,
}

fn main() {
    use status::StatusGetter;

    let opt = Opt::from_args();
    let config = config::Config::new();

    let mut files = opt.dirs;

    files.extend(config.repositries);

    let files: Vec<_> = files.into_iter().unique().collect();

    let all_status = files
        .into_iter()
        .map(|p| {
            let x = dirs::get_dirs(p);
            StatusGetter::new(&x).to_statues()
        })
        .flatten()
        .unique()
        .sorted_by(|a, b| Ord::cmp(&a.status, &b.status));

    let clean = all_status.clone().filter(|x| x.is_clean()).collect_vec();
    if !clean.is_empty() {
        println!("✅ Clean");
        let clean_files = clean
            .iter()
            .map(|s| s.file.file_stem().unwrap().to_str().unwrap())
            .collect_vec()
            .join(",");
        println!("\t{clean_files}");
    }

    let unclean = all_status.filter(|x| !x.is_clean()).collect_vec();
    if !unclean.is_empty() {
        println!("\n❌ Unclean");
        for text in unclean {
            println!("\t{}", text)
        }
    }
}
