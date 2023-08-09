#[allow(unused_imports)]
use log::{debug, error, info};

use file_patcher::setting::ClientConfig;
use file_patcher::FilePatcher;
use std::path::Path;
use url::{ParseError, Url};

pub fn get_client_config(path: Option<&Path>) -> ClientConfig {
    let config = match path {
        Some(path) => {
            info!("从 {} 中读取配置...", path.display());
            let config = ClientConfig::load_client_config(path);
            config
        }
        _ => {
            let config_path = Path::new("Client.toml");
            info!("从默认位置读取 Client.toml");

            let config = ClientConfig::load_client_config(config_path);
            if !config_path.exists() {
                config.generate_client_config(config_path);
            }
            config
        }
    };

    debug!("{:?}", &config);
    config
}

pub fn get_files_list() {}
