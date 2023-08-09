#[allow(unused_imports)]
use log::{debug, error, info, warn};

use std::thread::sleep;
use std::time::Duration;

pub fn countdown(seconds: u32) {
    for i in (1..=seconds).rev() {
        warn!("将会在 {} 秒后退出", i);
        sleep(Duration::from_secs(1));
    }
    info!("退出");
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
