use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use serde::{Deserialize, Serialize};

use crate::json::Json;

#[derive(Deserialize, Serialize)]
pub struct ConfigToml {
    pub ui: Ui,
}

#[derive(Deserialize, Serialize)]
pub struct Ui {
    pub show_help: bool,
}

pub struct Config;

static CONFIG_FILE_NAME: &str = "config";

impl Config {
    fn get_default() -> ConfigToml {
        ConfigToml {
            ui: Ui { show_help: true },
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = PathBuf::new();
        path.push(Json::get_dir_path().as_path());
        path.push(format!("{CONFIG_FILE_NAME}.toml"));

        return path;
    }

    pub fn read() -> ConfigToml {
        let path = Config::get_config_path();
        let config_raw = match fs::read_to_string(&path) {
            Ok(c) => c,
            // If config.toml file doesn't exist, create it by default
            Err(_) => {
                let default_config = toml::to_string(&Config::get_default()).unwrap();

                let mut file = File::create(&path).unwrap();
                let _ = file.write_all(default_config.as_bytes());

                default_config
            }
        };

        let data: ConfigToml = match toml::from_str(&config_raw) {
            Ok(c) => c,
            // If config.toml is not valid, throw a error message
            Err(_) => {
                eprint!(
                    "{} - ERROR: The configuration file is invalid. Please check the wiki for correct formatting or delete the file",
                    env!("CARGO_PKG_NAME")
                );
                exit(1)
            }
        };

        return data;
    }
}
