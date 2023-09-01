use crate::config::ServerConfig;
use rocket::fs::TempFile;
use rocket::tokio::fs;
use rocket::State;
use rr_updater::RUpdater;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct ListApi {
    pub result: u8,
    pub content: Option<RUpdater>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateApi {
    pub result: u8,
}

#[derive(Serialize, Deserialize)]
pub struct UploadApi {
    pub result: u8,
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

pub async fn upload_file(
    name: &str,
    path: PathBuf,
    mut file: TempFile<'_>,
    config: &State<ServerConfig>,
) -> UploadApi {
    let c = config.rr_config.iter().find(|n| n.name == name.to_string());
    match c {
        Some(f) => {
            todo!();
        }
        None => {}
    }

    let res = file.persist_to("./temp").await;
    match res {
        Ok(_) => UploadApi { result: 1 },
        Err(e) => {
            warn!("{}", e);
            UploadApi { result: 0 }
        }
    }
}
