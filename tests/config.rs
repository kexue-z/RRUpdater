#[cfg(test)]
mod tests {
    // #[test]
    // fn tests_server_config() {
    //     use file_patcher::setting::{Filesdir, ServerConfig};
    //     use std::path::Path;

    //     let config = ServerConfig::load_server_config(Path::new("./tests/server.toml"));

    //     assert_eq!(config.server.port, Some(1234));
    //     assert_eq!(config.server.key, "abcd");
    // }

    #[test]
    fn test_generate_server_config() {
        use file_patcher::setting::ServerConfig;
        use std::path::Path;
        ServerConfig::generate_server_config(Path::new("./tests/server.toml"));
    }
}
