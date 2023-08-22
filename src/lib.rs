use crypto::digest::Digest;
use crypto::sha1::Sha1;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub mod helper;
pub mod setting;

use setting::Filesdir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RUpdater {
    pub name: String,
    pub path: PathBuf,
    pub file_data: Vec<FileData>,
}

impl RUpdater {
    /// 一个文件夹
    pub fn new(filesdir: Filesdir) -> RUpdater {
        let dir: PathBuf = PathBuf::from(filesdir.path).iter().collect();
        // 如果文件夹不存在，则创建
        if !dir.exists() {
            fs::create_dir_all(&dir).unwrap();
        }

        let file_data: Vec<FileData> = Self::iter_path(dir.clone());

        RUpdater {
            name: filesdir.name,
            path: dir,
            file_data,
        }
    }

    pub fn save_updater_data(&self, path: &Path) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();

        fs::write(path, json_string).unwrap();
    }

    /// 遍历指定路径，返回路径下所有文件的信息
    /// -> Vec<FileData>
    fn iter_path(path: PathBuf) -> Vec<FileData> {
        let mut file_data = Vec::new();

        for e in WalkDir::new(&path) {
            let e: PathBuf = e.unwrap().into_path();
            if e.is_file() {
                file_data.push(FileData::new(path.to_path_buf(), e))
            }
        }
        file_data
    }

    pub fn read_json(raw_json: &str) -> RUpdater {
        serde_json::from_str(raw_json).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileData {
    pub name: String,
    pub path: PathBuf,
    pub sha1: String,
}

impl FileData {
    fn new(base: PathBuf, path: PathBuf) -> FileData {
        let name: String = path.file_name().unwrap().to_owned().into_string().unwrap();
        let path = path.iter().collect();
        let sha1: String = Self::calculate_sha1(&path);
        FileData {
            name,
            path: path.strip_prefix(base).unwrap().to_path_buf(),
            sha1,
        }
    }

    /// # 计算文件Sha1
    fn calculate_sha1(path: &PathBuf) -> String {
        let file: fs::File = fs::File::open(path).unwrap();
        let mut reader: BufReader<fs::File> = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        let mut hasher = Sha1::new();
        hasher.input(&buffer);
        hasher.result_str()
    }
}
