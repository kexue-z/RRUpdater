#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::tokio::fs;

use file_patcher::setting::ServerConfig;
use file_patcher::FilePatcher;

use server::{get_files_path, update_hash};
use server::{ListApi, UpdateApi};

use std::path::{Path, PathBuf};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

/// 获取文件
#[get("/<name>/<file..>")]
async fn files(name: &str, file: PathBuf) -> Option<NamedFile> {
    let files_path = get_files_path(name).await;

    if files_path.len() == 0 {
        None
    } else {
        NamedFile::open(Path::new(&files_path).join(file))
            .await
            .ok()
    }
}

#[get("/list/<name>")]
async fn files_list(name: &str) -> Json<ListApi> {
    let config = ServerConfig::async_load_server_config(Path::new("Server.toml"))
        .await
        .unwrap();

    let data_path = config.data_path;
    let path = format!("{}/{}.json", data_path, name);
    let file_path = Path::new(path.as_str());

    let file_content = fs::read_to_string(file_path).await;

    match file_content {
        Ok(content) => {
            let a = ListApi {
                result: 1,
                content: Some(FilePatcher::read_json(&content)),
            };
            Json(a)
        }
        Err(_) => {
            let a = ListApi {
                result: 0,
                content: None,
            };
            Json(a)
        }
    }
}

#[get("/update")]
async fn update() -> Json<UpdateApi> {
    update_hash().await;
    Json(UpdateApi { retult: 1 })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, files, files_list, update])
        .launch()
        .await?;

    Ok(())
}
