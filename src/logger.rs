use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::LevelFilter;
use std::{io, time::SystemTime};

pub fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Cyan)
        .trace(Color::Magenta);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                record.target(),
                message
            ));
        })
        .level(LevelFilter::Debug)
        .level_for("russh", LevelFilter::Info)
        .chain(io::stdout())
        .apply()?;

    Ok(())
}
