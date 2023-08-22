#[cfg(test)]
mod tests {
    #[test]
    fn test_updater() {
        use rr_updater::setting::ServerConfig;
        use rr_updater::RUpdater;

        use std::path::Path;

        let config = ServerConfig::load_server_config(Path::new("./tests/server.toml"));

        for i in config.server.files {
            let file_name = i.name.clone() + ".json";

            let patcher = RUpdater::new(i);

            let path = Path::new("./tests/data/").join(file_name);
            patcher.save_updater_data(&path);
        }
    }
}
