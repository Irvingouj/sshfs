use std::path::PathBuf;

use tracing::{info, level_filters::LevelFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn init_file_log(path: &Option<PathBuf>) -> impl Drop {
    let default_path = PathBuf::from("./debug");
    let path = path.as_ref().unwrap_or(&default_path);
    let file_appender = RollingFileAppender::new(Rotation::DAILY, path, "sshfs.log");

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .parse("")
        .expect("Failed to parse filter");

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_level(true)
        .with_file(true)
        .with_filter(env_filter);

    tracing_subscriber::registry()
        .with(file_layer) // Add the file layer
        .init();

    info!(?path, "log initialized");

    guard
}

