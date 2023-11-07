use serde::Deserialize;
use std::path::{Path, PathBuf};
use crate::expand::*;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub repositries: Vec<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        let config = expand("~/.gs.toml");

        if config.exists() {
            let text = std::fs::read_to_string(config).unwrap();
            let config: Config = toml::from_str(&text).unwrap();

            let repositries :Vec<_> = config.repositries.iter().map(expand_path).collect();

            Self {
                repositries,
                ..config
            }
            
        } else {
            println!("Can't find {:?}", config);
            Self {
                repositries: vec![],
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
