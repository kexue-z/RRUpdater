use crypto::digest::Digest;
use crypto::sha1::Sha1;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub mod setting;

#[derive(Serialize, Deserialize)]
pub struct FilePatcher {
    pub name: String,
    pub path: PathBuf,
    pub file_data: Vec<FileData>,
}

impl FilePatcher {
    /// 新建实例
    pub fn new(name: String, path: &PathBuf) -> FilePatcher {
        let dir_list: Vec<PathBuf> = Self::dir_list(&path);

        let mut file_data: Vec<FileData> = Vec::new();

        for dir in dir_list {
            let mut p: Vec<FileData> = Self::iter_path(dir);
            file_data.append(&mut p);
        }

        FilePatcher {
            name,
            path: path.to_path_buf(),
            file_data,
        }
    }

    pub fn save_file_patcher_data(&self, path: &Path) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path, json_string).unwrap();
    }

    /// 寻找包括 INCLUDE 的文件夹
    /// -> Vec<PathBuf>
    fn dir_list(path: &PathBuf) -> Vec<PathBuf> {
        let pattern: PathBuf = path.join("**/INCLUDE");

        let mut dirs: Vec<PathBuf> = Vec::new();
        for e in glob(pattern.to_str().unwrap())
            .unwrap()
            .filter_map(Result::ok)
        {
            let files_path: PathBuf = e.as_path().parent().unwrap().to_owned();
            dirs.push(files_path);
        }

        dirs
    }

    /// 遍历指定路径，返回路径下所有文件的信息
    /// -> Vec<FileData>
    fn iter_path(path: PathBuf) -> Vec<FileData> {
        let mut file_data = Vec::new();

        for e in WalkDir::new(path) {
            let e: PathBuf = e.unwrap().into_path();
            if e.file_name().unwrap() == "INCLUDE" {
                continue;
            }
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
