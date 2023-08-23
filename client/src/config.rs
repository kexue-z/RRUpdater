use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml::from_str;

#[derive(Deserialize, Serialize, Debug)]
pub struct Client {
    pub host: String,
    pub key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sync {
    pub name: String,
    pub to_path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientConfig {
    pub client: Client,
    pub sync: Vec<Sync>,
}

impl ClientConfig {
    pub fn load_client_config(path: &Path) -> ClientConfig {
        let f = fs::read_to_string(path);
        match f {
            Ok(s) => from_str(&s).unwrap(),
            Err(_) => Self::default(),
        }
    }

    pub fn generate_client_config(&self, path: &Path) {
        let toml = toml::to_string(&self).unwrap();
        fs::write(path, toml).unwrap();
    }

    pub fn default() -> ClientConfig {
        ClientConfig {
            client: Client {
                host: "http://127.0.0.1:8520".to_string(),
                key: "".to_string(),
            },
            sync: vec![],
        }
    }
}
