#[allow(unused_imports)]
use log::{debug, error, info};

use crate::utils::{compare_and_find, FPItems};
use reqwest::blocking::Client as WebClient;
use rr_updater::setting::{ClientConfig, Filesdir, Sync};
use rr_updater::RUpdater;
use serde::{Deserialize, Serialize};
use std::path::Path;
// use tempfile::tempdir;
use url::Url;

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

pub fn get_files_list(base_url: &Url, config: &ClientConfig, local: &RUpdater) -> FPItems {
    let client = WebClient::new();

    let sync_list = &config.sync;

    let mut items = FPItems { items: vec![] };

    for i in sync_list {
        // 遍历每个 Sync 设置
        let name = i.name.as_str();
        let url = base_url.join(&format!("list/{}", name)).unwrap();

        debug!("Requ url: {}", &url);
        // 发送请求
        let res = client.get(url).send();
        match res {
            Ok(r) => {
                // 请求成功
                let server_fp = r.json::<ListApi>().unwrap();

                debug!("{:?}", &server_fp);
                match server_fp.content {
                    Some(content) => {
                        items.items.push(compare_and_find(content, local.clone()));
                    }
                    None => {
                        error!("服务端未找到相应配置: {}", &name);
                    }
                }
            }
            Err(e) => {
                // 请求失败
                error!("{:?}", e);
            }
        }
    }

    debug!("{:?}", &items);

    items
}

pub fn update_file(sync: &Sync) -> RUpdater {
    // let name = sync.name.clone();
    info!("生成 {} 的数据", &sync.name);
    let fp = RUpdater::new(Filesdir {
        name: sync.name.clone(),
        path: sync.to_path.clone(),
    });

    // let tempdir = tempdir().unwrap();

    // if !data_path.exists() {
    //     info!("数据目录不存在, 新建目录位于 -> {}", data_path.display());
    //     fs::create_dir(data_path).unwrap();
    // }

    // let _data_path = &tempdir.path().join(format!("{}.json", name));
    // info!("保存生成文件位于 -> {}", _data_path.display());
    // fp.save_updater_data(_data_path);
    fp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListApi {
    pub result: u8,
    pub content: Option<RUpdater>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateApi {
    pub retult: u8,
}
