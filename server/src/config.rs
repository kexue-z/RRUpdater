use rocket::serde::{Deserialize, Serialize};
use rr_updater::Filesdir;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ServerConfig {
    pub key: String,
    pub data_path: PathBuf,
    pub rr_config: Vec<Filesdir>,
}
