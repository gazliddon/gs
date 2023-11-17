#![allow(dead_code)]
#![deny(unused_imports)]
mod config;
mod dirs;
mod expand;
mod status;

use itertools::Itertools;
use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;
use log::info;

#[derive(StructOpt, Debug)]
#[structopt(name = "gs")]
#[structopt(version="0.1.0")]
#[structopt(about="git status checker")]
#[structopt(author="gazaxian")]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_str = expand::expand_path))]
    dirs: Vec<PathBuf>,
}


fn main() -> Result<()> {
    use status::StatusGetter;

    let opt = Opt::from_args();

    if opt.debug {
        simple_logger::init_with_level(log::Level::Trace).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Error).unwrap();
    }

    info!("Starting with logging");

    let config = config::Config::new();

    let mut files = opt.dirs;

    files.extend(config.repositries);

    let files: Vec<_> = files.into_iter().unique().collect();

    let all_status = files
        .into_iter()
        .map(|p| {
            let x = dirs::get_dirs(p).expect("What");
            StatusGetter::new(&x).to_statues()
        })
        .flatten()
        .unique()
        .sorted_by(|a, b| Ord::cmp(&a.status, &b.status));

    let clean = all_status.clone().filter(|x| x.is_clean()).collect_vec();

    if clean.len() == all_status.len() {
        println!("✅ All Clean");
    } else {
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
    Ok(())
}
