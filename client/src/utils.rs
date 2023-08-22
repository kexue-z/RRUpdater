#[allow(unused_imports)]
use log::{debug, error, info, warn};

use rr_updater::{FileData, RUpdater};
use std::fs;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct FPItems {
    pub items: Vec<FPItem>,
}

#[derive(Debug)]
pub struct FPItem {
    pub name: String,
    pub base_path: PathBuf,
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

pub fn find_differences(a: &[FileData], b: &[FileData]) -> (Vec<FileData>, Vec<FileData>) {
    let mut missing_files: Vec<FileData> = Vec::new();
    let mut extra_files: Vec<FileData> = Vec::new();

    // Find missing files in b compared to a
    for file_a in a {
        let mut found = false;
        for file_b in b {
            if file_a.name == file_b.name
                && file_a.path == file_b.path
                && file_a.sha1 == file_b.sha1
            {
                found = true;
                break;
            }
        }
        if !found {
            missing_files.push(file_a.clone());
        }
    }

    // Find extra files in b compared to a
    for file_b in b {
        let mut found = false;
        for file_a in a {
            if file_b.name == file_a.name
                && file_b.path == file_a.path
                && file_b.sha1 == file_a.sha1
            {
                found = true;
                break;
            }
        }
        if !found {
            extra_files.push(file_b.clone());
        }
    }

    (missing_files, extra_files)
}

// / 读取本地保存的数据
// fn load_updater_data(name: &str) -> RUpdater {
//     let local_file_name = format!("{}.json", &name).as_str().to_owned();

//     let tempdir = tempdir().unwrap();
//     let f2 = tempdir.path().join(local_file_name);
//     let file_content = fs::read_to_string(f2).unwrap();

//     serde_json::from_str(file_content.as_str()).unwrap()
// }

/// 服务端与本地对比文件，获取差异列表
pub fn find_items(base_content: RUpdater, content: RUpdater) -> FPItem {
    let mut items = FPItem {
        name: content.name.to_owned(),
        base_path: content.path.to_owned(),
        missing: vec![],
        surplus: vec![],
    };

    let (missing_files, extra_files) =
        find_differences(&base_content.file_data, &content.file_data);

    items.missing = missing_files;
    items.surplus = extra_files;
    // // 找到缺失的项目
    // items
    //     .missing
    //     .append(&mut find_missing_items(&base_content, &content));

    // // 找到多余的项目
    // items
    //     .surplus
    //     .append(&mut find_surplus_items(&base_content, &content));

    items
}

pub fn compare_and_find(content: RUpdater, local: RUpdater) -> FPItem {
    // let local_file = load_updater_data(&name);

    find_items(content, local)
}

pub fn remove_files(items: &FPItems) {
    for i in &items.items {
        let files_list = i.surplus.to_owned();
        let base_path = i.base_path.to_owned();

        for f in files_list {
            let path = base_path.join(&f.path);

            let r = fs::remove_file(&path);
            match r {
                Ok(_) => {
                    info!("删除文件 => {}", path.display());
                }
                Err(e) => {
                    warn!("删除文件失败 => {} 原因: {}", path.display(), e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_find() {
        use super::find_differences;
        use rr_updater::FileData;
        use std::path::PathBuf;

        let a = vec![
            FileData {
                name: "a.txt".to_string(),
                path: PathBuf::from("a.txt"),
                sha1: "a".to_string(),
            },
            FileData {
                name: "b.txt".to_string(),
                path: PathBuf::from("dir/b.txt"),
                sha1: "b".to_string(),
            },
        ];

        let b = vec![
            FileData {
                name: "b.txt".to_string(),
                path: PathBuf::from("dir/b.txt"),
                sha1: "b".to_string(),
            },
            FileData {
                name: "c.txt".to_string(),
                path: PathBuf::from("dir/c.txt"),
                sha1: "c".to_string(),
            },
        ];

        let (missing_files, extra_files) = find_differences(&a, &b);

        println!("Missing Files:");
        for file in missing_files {
            println!("{:?}", file);
        }

        println!("Extra Files:");
        for file in extra_files {
            println!("{:?}", file);
        }
    }
}
