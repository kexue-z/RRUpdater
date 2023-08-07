#[cfg(test)]
mod tests {
    #[test]
    fn test_filepatcher() {
        use file_patcher::setting::ServerConfig;
        use file_patcher::FilePatcher;

        use std::path::Path;

        let config = ServerConfig::load_server_config(Path::new("./tests/server.toml"));

        for i in config.server.files {
            let file_name = i.name.clone() + ".json";

            let patcher = FilePatcher::new(i);

            let path = Path::new("./tests/data/").join(file_name);
            patcher.save_file_patcher_data(&path);
        }
    }
}
