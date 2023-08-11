mod source;
mod utils;

use crate::source::{get_client_config, get_files_list, update_file};
use crate::utils::{countdown, download_files, remove_files, setup_logger};
use clap::{Parser, Subcommand};
use log::LevelFilter;
use log::{debug, error, info, warn};
use std::path::{Path, PathBuf};
use url::Url;

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
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    no_run: bool,

    /// 不对文件进行操作
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    dry_run: bool,

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

    let _key = config.client.key.clone();

    let data_path = Path::new(&config.client.data_path);

    let syncs = &config.sync;

    if cli.no_run {
        warn!("跳过 SHA 计算");
    } else {
        if syncs.len() == 0 {
            error!("没有配置文件目录");
            countdown(5);
        }
        for sync in syncs {
            // 每个配置
            update_file(sync, data_path);
        }
    }

    let files_items = get_files_list(host, &config);
    if !cli.dry_run {
        remove_files(&files_items);
    }

    Ok(())
}
