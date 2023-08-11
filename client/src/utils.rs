use file_patcher::setting::ClientConfig;
#[allow(unused_imports)]
use log::{debug, error, info, warn};

use file_patcher::{FileData, FilePatcher};
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::Path};

pub struct FPItems {
    pub missing: Vec<FileData>,
    pub surplus: Vec<FileData>,
}

pub fn countdown(seconds: u32) {
    for i in (1..=seconds).rev() {
        warn!("将会在 {} 秒后退出", i);
        sleep(Duration::from_secs(1));
    }
    info!("退出");
}

pub fn setup_logger(log_level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

/// 比较两个，获取缺失的 FileData
/// f1 为基准
pub fn find_missing_items<'a>(f1: &'a FilePatcher, f2: &'a FilePatcher) -> Vec<FileData> {
    let mut missing_items: Vec<FileData> = Vec::new();

    let data1 = &f1.file_data;
    let data2 = &f2.file_data;

    let sha1s_a: Vec<String> = data1.iter().map(|s| s.sha1.to_owned()).collect();
    let sha1s_b: Vec<String> = data2.iter().map(|s| s.sha1.to_owned()).collect();

    for sha1 in sha1s_a {
        if !sha1s_b.contains(&sha1) {
            let a = data1.iter().find(|s| s.sha1 == sha1).unwrap();
            missing_items.push(a.clone());
        }
    }

    missing_items
}

/// 比较两个，获取多余的 FileData
/// f1 为基准
pub fn find_surplus_items<'a>(f1: &'a FilePatcher, f2: &'a FilePatcher) -> Vec<FileData> {
    let mut surplus_items: Vec<FileData> = Vec::new();

    let data1 = &f1.file_data;
    let data2 = &f2.file_data;

    // surplus_items.append(&mut data2.clone());

    let sha1s_a: Vec<String> = data1.iter().map(|s| s.sha1.to_owned()).collect();
    let sha1s_b: Vec<String> = data2.iter().map(|s| s.sha1.to_owned()).collect();

    for sha1_b in &sha1s_b {
        for sha1_a in &sha1s_a {
            if sha1_a == sha1_b {
                continue;
            } else {
                let a = data2
                    .iter()
                    .find(|s| s.sha1.to_owned() == sha1_b.to_owned())
                    .clone()
                    .unwrap();
                surplus_items.push(a.clone());
            }
        }
    }

    surplus_items
}

/// 读取本地保存的数据
fn load_local_file_patcher_data(config: &ClientConfig, name: &str) -> FilePatcher {
    let local_file_name = format!("{}.json", &name).as_str().to_owned();

    let f2 = Path::new(&config.client.data_path).join(local_file_name);
    let file_content = fs::read_to_string(f2).unwrap();

    serde_json::from_str(file_content.as_str()).unwrap()
}

/// 服务端与本地对比文件，获取差异列表
pub fn find_items(base_content: FilePatcher, content: FilePatcher) -> FPItems {
    let mut items = FPItems {
        missing: vec![],
        surplus: vec![],
    };

    // 找到缺失的项目
    items
        .missing
        .append(&mut find_missing_items(&base_content, &content));

    // 找到多余的项目
    items
        .surplus
        .append(&mut find_surplus_items(&base_content, &content));

    items
}

pub fn compare_and_find(content: FilePatcher, name: &str, config: &ClientConfig) -> FPItems {
    let local_file = load_local_file_patcher_data(&config, &name);

    find_items(content, local_file)
}

pub fn download_files(items: FPItems) {
    todo!();
}

pub fn remove_files(items: &FPItems) {
    let files_list = items.surplus.to_owned();
    for f in files_list {
        let r = fs::remove_file(&f.path);
        match r {
            Ok(_) => {
                info!("删除文件 => {}", &f.path.display());
            }
            Err(e) => {
                warn!("删除文件失败 => {} 原因: {}", &f.path.display(), e);
            }
        }
    }
}
