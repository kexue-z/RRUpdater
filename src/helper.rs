use crate::setting::ServerConfig;
use std::fs;
use std::path::Path;

/// 初始化工作目录
pub fn init_dir() {
    let default_data_path = Path::new("./data");
    if !default_data_path.exists() {
        fs::create_dir(default_data_path).unwrap();
    }

    let dafault_config = ServerConfig::default();
    dafault_config.generate_server_config(Path::new("./Server.toml"));
}
