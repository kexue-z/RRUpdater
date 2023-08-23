use rocket::serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ServerConfig {
    pub key: String,
    pub data_path: PathBuf,
    pub rr_config: Vec<Filesdir>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Filesdir {
    pub name: String,
    pub path: String,
}
