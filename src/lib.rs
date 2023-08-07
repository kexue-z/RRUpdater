use crypto::digest::Digest;
use crypto::sha1::Sha1;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub mod setting;

use setting::Filesdir;

#[derive(Serialize, Deserialize)]
pub struct FilePatcher {
    pub name: String,
    pub path: PathBuf,
    pub file_data: Vec<FileData>,
}

impl FilePatcher {
    /// 一个文件夹
    pub fn new(filesdir: Filesdir) -> FilePatcher {
        let dir: PathBuf = PathBuf::from(filesdir.path).iter().collect();
        let file_data: Vec<FileData> = Self::iter_path(dir.clone());

        FilePatcher {
            name: filesdir.name,
            path: dir,
            file_data,
        }
    }

    pub fn save_file_patcher_data(&self, path: &Path) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path, json_string).unwrap();
    }

    /// 遍历指定路径，返回路径下所有文件的信息
    /// -> Vec<FileData>
    fn iter_path(path: PathBuf) -> Vec<FileData> {
        let mut file_data = Vec::new();

        for e in WalkDir::new(path) {
            let e: PathBuf = e.unwrap().into_path();
            if e.is_file() {
                file_data.push(FileData::new(e))
            }
        }
        file_data
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileData {
    pub name: String,
    pub path: PathBuf,
    pub sha1: String,
}

impl FileData {
    fn new(path: PathBuf) -> FileData {
        let name: String = path.file_name().unwrap().to_owned().into_string().unwrap();
        let path = path.iter().collect();
        let sha1: String = Self::calculate_sha1(&path);
        FileData { name, path, sha1 }
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
