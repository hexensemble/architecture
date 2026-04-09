use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::{self, OpenOptions};
use std::path::PathBuf;

pub fn init_logging(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = PathBuf::from("logs");
    fs::create_dir_all(&log_dir)?;
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join(filename))?;

    let config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Info)
        .set_level_padding(LevelPadding::Right)
        .build();

    let file_logger = WriteLogger::new(LevelFilter::Info, config.clone(), log_file);

    let term_logger = TermLogger::new(
        LevelFilter::Info,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    CombinedLogger::init(vec![file_logger, term_logger])?;

    Ok(())
}
