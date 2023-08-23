use rocket::tokio::fs;
use rr_updater::setting::ServerConfig;
use rr_updater::RUpdater;
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

    let data_path = Path::new(&config.data_path);

    if !data_path.exists() {
        fs::create_dir_all(&data_path).await.unwrap();
    }

    for f in file {
        let name = f.name.clone();
        let name = name + ".json";

        let patcher = RUpdater::new(f);
        let path = data_path.join(name);

        patcher.save_updater_data(&path);
    }
}
