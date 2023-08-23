mod config;
mod utils;

#[macro_use]
extern crate rocket;

use config::ServerConfig;
use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::tokio::fs;
use rocket::State;
use rr_updater::RUpdater;
use std::path::{Path, PathBuf};
use utils::update_hash;
use utils::{ListApi, UpdateApi};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

/// 获取文件
#[get("/<name>/<file..>")]
async fn files(name: &str, file: PathBuf, config: &State<ServerConfig>) -> Option<NamedFile> {
    let filesdirs_list = &config.rr_config;

    let files_dir = filesdirs_list.iter().find(|filesdir| filesdir.name == name);

    match files_dir {
        Some(name_dir) => NamedFile::open(Path::new(&name_dir.path).join(file))
            .await
            .ok(),
        None => None,
    }
}

#[get("/list/<name>")]
async fn files_list(name: &str, config: &State<ServerConfig>) -> Json<ListApi> {
    let data_path = config.data_path.to_owned();

    let file_path = data_path.join(format!("{}.json", name));

    let file_content = fs::read_to_string(file_path).await;

    match file_content {
        Ok(content) => {
            let a = ListApi {
                result: 1,
                content: Some(RUpdater::read_json(&content)),
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

#[post("/update?<key>")]
async fn update(key: String, config: &State<ServerConfig>) -> Json<UpdateApi> {
    if config.key == key {
        update_hash(config).await;
        Json(UpdateApi { result: 1 })
    } else {
        Json(UpdateApi { result: 0 })
    }
}

// async fn timer(config: &State<ServerConfig>) {
//     let _task = task::spawn(async {
//         // 创建一个每隔12小时运行一次的定时器
//         let mut interval = interval(std::time::Duration::from_secs(60 * 60 * 12));

//         loop {
//             interval.tick().await;
//             println!("更新 hash");
//             update_hash(config).await;
//         }
//     });
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // timer().await;

    let _rocket = rocket::build()
        .mount("/", routes![index, files, files_list, update])
        .attach(AdHoc::config::<ServerConfig>())
        .launch()
        .await?;

    Ok(())
}
