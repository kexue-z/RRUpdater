use serde::{Deserialize, Serialize};
use std::fs;

use std::path::Path;
use tokio::fs::read_to_string;
use toml::from_str;

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfig {
    pub server: Server,
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
        let f = fs::read_to_string(path).expect("File to read config file");
        from_str(&f).unwrap()
    }

    pub async fn async_load_server_config(
        path: &Path,
    ) -> Result<ServerConfig, Box<dyn std::error::Error>> {
        let file_content = read_to_string(path).await?;
        let server_config = from_str(&file_content)?;
        Ok(server_config)
    }

    pub fn generate_server_config(self, path: &Path) {
        let toml = toml::to_string(&self).unwrap();
        fs::write(path, toml).unwrap();
    }
}
