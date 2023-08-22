use rr_updater::setting::ServerConfig;
use rr_updater::RUpdater;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tempfile::tempdir;

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
    pub content: Option<RUpdater>,
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

    // let data_path = config.data_path;
    let tempdir = tempdir().unwrap();

    for f in file {
        let name = f.name.clone();
        let path = format!("{}/{}.json", tempdir.path().display(), name);
        let patcher = RUpdater::new(f);
        let path = Path::new(&path);
        patcher.save_updater_data(&path);
    }
    todo!();
}
