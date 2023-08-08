use serde::{Deserialize, Serialize};
use std::fs;

use std::path::Path;
use tokio::fs::read_to_string as a_read_to_string;
use toml::from_str;

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfig {
    pub data_path: String,
    pub server: Server,
    pub key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    pub files: Vec<Filesdir>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Filesdir {
    pub name: String,
    pub path: String,
}

impl ServerConfig {
    pub fn load_server_config(path: &Path) -> ServerConfig {
        let f = fs::read_to_string(path);
        match f {
            Ok(s) => from_str(&s).unwrap(),
            Err(_) => {
                // 如无 Server.toml 则新建
                Self::default()
            }
        }
    }

    pub async fn async_load_server_config(
        path: &Path,
    ) -> Result<ServerConfig, Box<dyn std::error::Error>> {
        let file_content = a_read_to_string(path).await?;
        let server_config = from_str(&file_content)?;
        Ok(server_config)
    }

    pub fn generate_server_config(&self, path: &Path) {
        let toml = toml::to_string(&self).unwrap();
        fs::write(path, toml).unwrap();
    }

    pub fn default() -> ServerConfig {
        ServerConfig {
            data_path: "./data".to_string(),
            server: Server { files: vec![] },
            key: "".to_string(),
        }
    }
}
