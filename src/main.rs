use file_patcher::FilePatcher;
use std::path::Path;

mod setting;
use setting::ServerConfig;

fn main() {
    let config = ServerConfig::load_server_config(Path::new("a.toml"));
    dbg!(config);
    // let path = Path::new(r"D:\收件箱\8");
    // let f = FilePatcher::new("a".to_string(), &path.to_path_buf());
    // println!("Path: {}", &f.path.to_str().unwrap());
    // f.save_file_patcher_data(Path::new("s.json"));
    // for f in f.file_data.iter() {
    //     println!("Filename: {}", f.name);
    //     println!("FilePath: {}", f.path.to_str().unwrap());
    //     println!("FileSha1: {}", f.sha1);
    //     println!("=====");
    // }

    // let files = generate_file_list(path);
    // dbg!(&files);
    // for f in files {
    //     let r = calculate_sha1(f);
    //     println!("{}", r);
    // }
    // println!("Hello, world!");
}
