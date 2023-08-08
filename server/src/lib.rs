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

#[derive(Serialize, Deserialize)]
pub struct UpdateApi {
    pub retult: u8,
}

pub async fn update_hash() {
    let config = ServerConfig::async_load_server_config(Path::new("Server.toml"))
        .await
        .unwrap();

    let file = config.server.files;

    for f in file {
        let name = f.name.clone();
        let path = format!("./file_patcher_data/{}.json", name);
        let patcher = FilePatcher::new(f);
        let path = Path::new(&path);
        patcher.save_file_patcher_data(&path);
    }
}
