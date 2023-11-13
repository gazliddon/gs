use crate::expand::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub repositries: Vec<PathBuf>,
}

fn find_config_files(home_dir_name: &str, non_home_dir_name: &str) -> Vec<PathBuf> {
    let to_check = vec![
        dirs::home_dir().map(|p| p.join(home_dir_name) ),
        dirs::config_local_dir().map(|p| p.join(non_home_dir_name)),
        dirs::config_dir().map(|p| p.join(non_home_dir_name))
    ];

    to_check
        .into_iter()
        .filter_map(std::convert::identity)
        .filter(|p| p.exists())
        .collect()
}

impl Config {
    pub fn new() -> Self {
        let configs = find_config_files(".gs.toml", "gs.toml");

        let config = configs.get(0);

        if let Some(config) = config {
            let text = std::fs::read_to_string(config).unwrap();
            let config: Config = toml::from_str(&text).unwrap();
            let repositries: Vec<_> = config.repositries.iter().map(expand_path).collect();
            Self {
                repositries,
                ..config
            }
        } else {
            Self {
                repositries: vec![],
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repositries: Default::default(),
        }
    }
}
