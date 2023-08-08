#[cfg(test)]
mod tests {

    #[test]
    fn test_generate_server_config() {
        use file_patcher::setting::{Filesdir, Server, ServerConfig};
        use std::path::Path;

        let config = ServerConfig {
            data_path: "./tests/data".to_string(),
            server: Server {
                files: vec![
                    Filesdir {
                        name: "files".to_string(),
                        path: "./tests/files".to_string(),
                    },
                    Filesdir {
                        name: "files1".to_string(),
                        path: "./tests/files1".to_string(),
                    },
                ],
            },
        };

        config.generate_server_config(Path::new("./tests/Server.toml"));
    }

    #[test]
    #[allow(unused_variables)]
    fn test_load_server_config() {
        use file_patcher::setting::{Filesdir, Server, ServerConfig};
        use std::path::Path;

        let config = ServerConfig::load_server_config(Path::new("./tests/Server.toml"));

        let config_example = ServerConfig {
            data_path: "./tests/data".to_string(),
            server: Server {
                files: vec![
                    Filesdir {
                        name: "files".to_string(),
                        path: "./tests/files".to_string(),
                    },
                    Filesdir {
                        name: "files1".to_string(),
                        path: "./tests/files1".to_string(),
                    },
                ],
            },
        };
        // 测试待完成
    }
}
