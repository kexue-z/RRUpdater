#[allow(unused_imports)]
use log::{debug, error, info};

use crate::utils::FPItems;
use bytes::Bytes;
use log::warn;
use reqwest::blocking::Client as WebClient;
use reqwest::StatusCode;
use reqwest::Url;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;

pub fn download_files(base_url: Url, items: FPItems) {
    let mut handles = vec![];

    for i in items.items {
        for j in i.missing {
            // URL = 基础URL + 名字 + 路径
            let url = base_url
                .clone()
                .join(&format!("{}/{}", &i.name, j.path.display()))
                .unwrap();
            // let url = path_url.join("test.png").unwrap();
            // .join(j.path.to_str().unwrap())
            // .unwrap();

            // 文件保存路径 = 基础路径 + 路径
            // let path = &

            let path = i.base_path.join(&j.path);
            handles.push(thread::spawn(move || {
                download_file(url, &path);
            }))
        }
    }
    for i in handles {
        let r = i.join();
        match r {
            Ok(_) => (),
            Err(e) => warn!("创建下载线程失败 => {:?}", e),
        }
    }
}

pub fn download_file(url: Url, path: &Path) {
    let client = WebClient::new();
    let _url = url.clone();
    let res = client.get(url).send();
    // let abs_path: Vec<_> = path.iter().collect();

    match res {
        Ok(r) => {
            if r.status() == StatusCode::OK {
                let b = r.bytes().unwrap();
                info!("下载新文件 => {} -> {}", _url, path.display());
                save_bytes(path, b);
            } else {
                error!("文件获取失败 => {}", _url);
            }
        }

        Err(e) => {
            error!("下载文件失败 => {}", e);
        }
    }
}

fn save_bytes(path: &Path, bytes: Bytes) {
    let path: PathBuf = path.to_owned().iter().collect();

    if !path.parent().unwrap().exists() {
        info!("创建文件夹 => {}", &path.parent().unwrap().display());
        fs::create_dir(&path.parent().unwrap()).unwrap();
    }

    let file = File::create(path);

    match file {
        Ok(mut f) => {
            f.write_all(&bytes).unwrap();
        }
        Err(e) => {
            warn!("创建文件失败 => {}", e);
        }
    }
}
