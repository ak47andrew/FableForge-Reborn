use std::fs;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use common::{dump, parse};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub id: u64,
}

impl Config {
    pub fn new() -> Self {
        Config {
            base_url: "ws://ffr.321657325.xyz/ws".to_string(),
            id: 0
        }
    }

    pub fn dump(&self) {
        let dumped = dump(self);
        fs::write(Config::get_path(), dumped).unwrap();
    }

    pub fn load() -> Self {
        match fs::read_to_string(Config::get_path()) {
            Err(_) => Self::new(),
            Ok(t) => {
                match parse::<Config>(t.as_str()) {
                    Ok(t) => {
                        t
                    }
                    Err(_) => {
                        let obj = Config::new();
                        obj.dump();
                        obj
                    }
                }
            }
        }
    }

    fn get_path() -> String {
        ProjectDirs::from("dev", "ak47andrew", "fableforge-reborn")
            .unwrap()
            .config_dir()
            .join("config.json")
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_conn_path(&self) -> String {
        format!(
            "{}/{}",
            self.base_url
                .strip_suffix("/").unwrap_or(&self.base_url),
            self.id
        )
    }
}
