#[allow(unused_imports)]
use log::{debug, error, info};

use crate::utils::FPItems;
use reqwest::blocking::Client as WebClient;
use reqwest::Url;
use std::path::Path;
use std::thread;

pub fn download_files(base_url: &Url, items: FPItems) {
    let mut handles = vec![];

    for i in items.missing {
        let download_path = i.path;
        handles.push(thread::spawn(move || {
            todo!();
        }))
    }
    todo!();
}

fn download_file(base_url: Url, path: Path) {
    let client = WebClient::new();
    let res = client.get(base_url).send();

    match res {
        Ok(r) => {
            todo!();
        }
        Err(e) => {
            error!("下载文件失败 => {}", e);
        }
    }

    todo!();
}
