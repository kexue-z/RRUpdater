mod utils;

#[allow(unused_imports)]
use log::{debug, error, info};

use clap::{Parser, Subcommand};

use client::get_client_config;
use std::path::{Path, PathBuf};
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
    setup_logger()?;

    let cli = Cli::parse();

    let config = get_client_config(cli.config.as_deref());
    // if let Some(config_path<'a>) = cli.config.as_deref() {
    //     let config_path = config_path;
    //     info!("从 {} 中读取配置...", config_path.display());
    // } else {
    //     let config_path = Path::new("Client.toml");
    //     info!("使用默认配置 Client.toml")
    // }

    // let config = get_client_config(config_path);

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => info!("Debug mode is OFF"),
        1 => debug!("Debug mode is on"),
        _ => error!("What r u doing?"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    // match &cli {
    //     Some(Commands::Test { list }) => {
    //         if *list {
    //             info!("Printing testing lists...");
    //         } else {
    //             info!("Not printing testing lists...");
    //         }
    //     }
    //     None => {}
    // }
    countdown(3);

    // Continued program logic goes here...
    Ok(())
}
