#![allow(dead_code)]
#![deny(unused_imports)]
mod config;
mod dirs;
mod expand;
mod status;

use itertools::Itertools;

use std::path::PathBuf;
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
    #[structopt(name = "FILE", parse(from_str = expand::expand_path))]
    files: Vec<PathBuf>,
}


fn main() {
    use status::StatusGetter;

    let opt = Opt::from_args();
    let config = config::Config::new();

    let mut files = opt.files;
    files.extend(config.repositries);

    let files: Vec<_> = files.into_iter().unique().collect();

    println!("{:?}", files);

    for p in files {
        let x = dirs::get_dirs(p);

        let statuses = StatusGetter::new(&x).to_statues();

        let txt_iter = statuses.into_iter().map(|s| s.to_string()).sorted();

        for text in txt_iter {
            println!("{}", text)
        }
    }
}
