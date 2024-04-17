use serde::{ Deserialize, Serialize };
use confy;

#[derive(Serialize, Deserialize)]
pub struct Config {
    key: Option<String>,
    endpoint: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            key: None,
            endpoint: None,
        }
    }
}

impl Config {
    pub fn get_key(&self) -> Result<&String, &'static str> {
        match &self.key {
            Some(k) => Ok(k),
            None => Err("key is not set"),
        }
    }

    pub fn get_endpoint(&self) -> Result<&String, &'static str> {
        match &self.endpoint {
            Some(e) => Ok(e),
            None => Err("endpoint is not set"),
        }
    }
    pub fn set_config(&mut self, kv: &[String; 2]) -> Result<(), confy::ConfyError> {
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
                    println!("Invalid argument, please use key or endpoint");
                    break;
                }
            }
        }
        confy::store("tts_cli_rs", "conf", self)?;
        Ok(())
    }

    pub fn get_config(&self, key: &String) -> Result<&String, &'static str> {
        match key.as_str() {
            "key" => self.get_key(),
            "endpoint" => self.get_endpoint(),
            _ => Err("Invalid argument, please use key or endpoint"),
        }
    }
}
