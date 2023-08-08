use file_patcher::setting::ServerConfig;
use file_patcher::FilePatcher;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub async fn get_files_path(name: &str) -> String {
    let config = ServerConfig::async_load_server_config(Path::new("Server.toml"))
        .await
        .unwrap();

    let file = config
        .server
        .files
        .iter()
        .find(|filedir| filedir.name == name);

    if let Some(filedir) = file {
        filedir.path.clone()
    } else {
        "".to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ListApi {
    pub result: u8,
    pub content: Option<FilePatcher>,
}
