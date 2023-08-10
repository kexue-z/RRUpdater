mod utils;

use log::warn;
#[allow(unused_imports)]
use log::{debug, error, info};

use clap::{Parser, Subcommand};

use client::{get_client_config, get_files_list, update_file};
use log::LevelFilter;
use std::path::{Path, PathBuf};
use url::Url;

use utils::{countdown, setup_logger};

#[derive(Parser)]
#[command(name = "FilePatcher Client")]
#[command(author = "kexue <xana278@foxmail.com>")]
#[command(version)]
#[command(about = "从服务端同步文件到本地", long_about = None)]
struct Cli {
    /// 指定配置文件
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// DEBUG 模式
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// 不计算 SHA
    #[arg(short, long, action = clap::ArgAction::Count)]
    no_run: u8,

    /// 操作
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 请求服务器进行更新
    UpdateServer {
        /// 服务器 key
        #[arg(short, long, value_name = "KEY")]
        key: String,
    },
    /// 更新文件
    Update {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.debug {
        0 => {
            setup_logger(LevelFilter::Info)?;
            info!("Debug mode is OFF");
        }
        1 => {
            setup_logger(LevelFilter::Debug)?;
            debug!("Debug mode is on");
        }
        _ => {
            setup_logger(LevelFilter::Debug)?;
            error!("What r u doing?");
        }
    }

    let config = get_client_config(cli.config.as_deref());

    let host = match Url::parse(config.client.host.as_str()) {
        Ok(r) => r,
        Err(_) => {
            let host = Url::parse("http://127.0.0.1:8520").unwrap();
            error!("host 配置错误, URL不合法");
            warn!("使用默认host: {}", &host);
            host
        }
    };

    let key = config.client.key.clone();

    let data_path = Path::new(&config.client.data_path);

    let syncs = &config.sync;

    match cli.no_run {
        0 => {
            if syncs.len() == 0 {
                error!("没有配置文件目录");
                countdown(5);
            }
            for sync in syncs {
                // 每个配置
                update_file(sync, data_path);
            }
        }
        _ => {
            warn!("跳过 SHA 计算");
        }
    }

    get_files_list(host, &config);

    Ok(())
}
