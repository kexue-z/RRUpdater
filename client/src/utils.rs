#[allow(unused_imports)]
use log::{debug, error, info, warn};

use file_patcher::{FileData, FilePatcher};
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

pub fn find_missing_items<'a>(f1: &'a FilePatcher, f2: &'a FilePatcher) -> Vec<FileData> {
    let mut missing_items: Vec<FileData> = Vec::new();

    let data1 = &f1.file_data;
    let data2 = &f2.file_data;

    let sha1s_a: Vec<String> = data1.iter().map(|s| s.sha1.to_owned()).collect();
    let sha1s_b: Vec<String> = data2.iter().map(|s| s.sha1.to_owned()).collect();

    for sha1 in sha1s_a {
        if !sha1s_b.contains(&sha1) {
            let a = data1.iter().find(|s| s.sha1 == sha1).unwrap();
            missing_items.push(a.clone());
        }
    }

    missing_items
}
