use crate::config::CONFIG;

use chrono::Local;
use fern::Dispatch;
use log::info;

/// Initializes the logging system using the `fern` crate, configuring both file and console outputs with a custom format.
pub fn init() -> Result<(), fern::InitError> {
    CONFIG.paths.ensure()?;

    let log_file =
        CONFIG.paths.dirs["logs"].join(format!("{}.log", Local::now().format("%Y-%m-%d_%H-%M-%S")));

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file(&log_file)?)
        .chain(std::io::stdout())
        .level(log::LevelFilter::Info)
        .level_for("uxs_lib", CONFIG.metadata.log_level)
        .apply()?;

    info!("日志系统初始化成功，日志文件路径: {}", log_file.display());
    Ok(())
}
