#[allow(unused_imports)]
use log::{debug, error, info};

use std::thread::sleep;
use std::time::Duration;

pub fn countdown(seconds: u32) {
    for i in (1..=seconds).rev() {
        info!("Remaining time: {} seconds", i);
        sleep(Duration::from_secs(1));
    }
    info!("Countdown complete!");
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
