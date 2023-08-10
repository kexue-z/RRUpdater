#[allow(unused_imports)]
use log::{debug, error, info};

use file_patcher::setting::{ClientConfig, Filesdir, Sync};
use file_patcher::FilePatcher;
use reqwest::blocking::Client as WebClient;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use url::Url;

pub mod utils;

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

pub fn get_files_list(base_url: Url, config: &ClientConfig) {
    let client = WebClient::new();

    let sync_list = &config.sync;

    for i in sync_list {
        let name = i.name.as_str();
        let url = base_url.join(&format!("list/{}", name)).unwrap();

        debug!("Requ url: {}", &url);
        let res = client.get(url).send();
        match res {
            Ok(r) => {
                // debug!("{:?}", r.text());
                let server_fp = r.json::<ListApi>().unwrap();
                // 读取本地
                debug!("{:?}", server_fp);
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }
}

pub fn update_file(sync: &Sync, data_path: &Path) {
    let name = sync.name.clone();
    info!("生成 {} 的数据", &sync.name);
    let fp = FilePatcher::new(Filesdir {
        name: sync.name.clone(),
        path: sync.to_path.clone(),
    });

    if !data_path.exists() {
        info!("数据目录不存在, 新建目录位于 -> {}", data_path.display());
        fs::create_dir(data_path).unwrap();
    }

    let _data_path = &data_path.join(format!("{}.json", name));
    info!("保存生成文件位于 -> {}", _data_path.display());
    fp.save_file_patcher_data(_data_path);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListApi {
    pub result: u8,
    pub content: Option<FilePatcher>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateApi {
    pub retult: u8,
}
