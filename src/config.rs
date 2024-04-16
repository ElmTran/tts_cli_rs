use serde::{ Deserialize, Serialize };
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    key: Option<String>,
    endpoint: Option<String>,
}

impl Config {
    pub fn from_toml(path: &str) -> Self {
        if !std::path::Path::new("conf.toml").exists() {
            let toml = toml::to_string(&(Config { key: None, endpoint: None })).unwrap();
            fs::write("conf.toml", toml).expect("Error writing file");
        }
        let toml = fs::read_to_string(path).expect("Error reading file");
        toml::from_str(&toml).unwrap()
    }

    pub fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    pub fn get_endpoint(&self) -> Option<String> {
        self.endpoint.clone()
    }

    pub fn set_config(&mut self, kv: &[String; 2]) {
        let mut iter = kv.iter();
        while let Some(k) = iter.next() {
            match k.as_str() {
                "key" => {
                    self.key = Some(iter.next().unwrap().to_string());
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
        if !std::path::Path::new("conf.toml").exists() {
            let toml = toml::to_string(&self).unwrap();
            fs::write("conf.toml", toml).expect("Error writing file");
        } else {
            let toml = fs::read_to_string("conf.toml").expect("Error reading file");
            let mut config: Config = toml::from_str(&toml).unwrap();
            if self.key.is_some() {
                config.key = self.key.clone();
            }
            if self.endpoint.is_some() {
                config.endpoint = self.endpoint.clone();
            }
            let toml = toml::to_string(&config).unwrap();
            fs::write("conf.toml", toml).expect("Error writing file");
        }
    }

    pub fn get_config(&self, key: &String) -> Option<String> {
        match key.as_str() {
            "key" => self.get_key(),
            "endpoint" => self.get_endpoint(),
            _ => None,
        }
    }
}
