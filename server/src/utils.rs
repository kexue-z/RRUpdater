use crate::config::ServerConfig;
use rocket::tokio::fs;
use rocket::State;
use rr_updater::RUpdater;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct ListApi {
    pub result: u8,
    pub content: Option<RUpdater>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateApi {
    pub retult: u8,
}

pub async fn update_hash(config: &State<ServerConfig>) {
    let file = &config.rr_config;

    let data_path = Path::new(&config.data_path);

    if !data_path.exists() {
        fs::create_dir_all(&data_path).await.unwrap();
    }

    for f in file {
        let name = f.name.clone();
        let name = name + ".json";

        let patcher = RUpdater::new(f.clone());
        let path = data_path.join(name);

        patcher.save_updater_data(&path);
    }
}
