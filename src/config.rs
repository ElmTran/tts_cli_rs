use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    key: Option<String>,
    region: Option<String>,
    endpoint: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            key: None,
            region: None,
            endpoint: None,
        }
    }
    pub fn set_config(&mut self, kv: &[String; 2]) {
        let mut iter = kv.iter();
        while let Some(k) = iter.next() {
            match k.as_str() {
                "key" => {
                    self.key = Some(iter.next().unwrap().to_string());
                }
                "region" => {
                    self.region = Some(iter.next().unwrap().to_string());
                }
                "endpoint" => {
                    self.endpoint = Some(iter.next().unwrap().to_string());
                }
                _ => {
                    println!("Invalid argument");
                    break;
                }
            }
        }
        if !std::path::Path::new("config.toml").exists() {
            let toml = toml::to_string(&self).unwrap();
            fs::write("config.toml", toml).expect("Error writing file");
        } else {
            let toml = fs::read_to_string("config.toml").expect("Error reading file");
            let mut config: Config = toml::from_str(&toml).unwrap();
            if self.key.is_some() {
                config.key = self.key.clone();
            }
            if self.region.is_some() {
                config.region = self.region.clone();
            }
            if self.endpoint.is_some() {
                config.endpoint = self.endpoint.clone();
            }
            let toml = toml::to_string(&config).unwrap();
            fs::write("config.toml", toml).expect("Error writing file");
        }
    }

    pub fn get_config(&self, key: String) {
        let toml = fs::read_to_string("config.toml").expect("Error reading file");
        let config: Config = toml::from_str(&toml).unwrap();
    }
}
