use std::path::Path;

use file_patcher::setting::ServerConfig;

pub async fn get_files_path(name: &str) -> String {
    let config = ServerConfig::async_load_server_config(Path::new("Server.toml"))
        .await
        .unwrap();

    let file = config
        .server
        .files
        .iter()
        .find(|filedir| filedir.name == name);

    if let Some(filedir) = file {
        filedir.path.clone()
    } else {
        "".to_string()
    }
}
