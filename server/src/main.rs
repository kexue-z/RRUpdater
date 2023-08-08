#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use server::get_files_path;
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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, files])
        .launch()
        .await?;

    Ok(())
}
