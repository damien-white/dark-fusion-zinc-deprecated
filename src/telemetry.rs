use std::{fs::File, sync::Arc};

use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt, prelude::*, Registry};

const CRATE_ROOT: &str = env!("CARGO_PKG_NAME");

/// Initialize tracing with a console logger and file logger.
pub fn init_tracing(default_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filter_level = default_level.parse().unwrap_or_else(|_| {
        eprintln!("Invalid level filter. Falling back to 'DEBUG'");
        Level::DEBUG
    });

    let log_file = File::create("zinc.log")?;

    let stdout_logger = fmt::layer();
    let file_logger = fmt::layer().with_writer(Arc::new(log_file)).json();

    Registry::default()
        // Send tracing output to the stdout logger based on the set filter level
        .with(stdout_logger.with_filter(Targets::default().with_default(filter_level)))
        // Send all tracing output allowed by global filter to the file logger
        .with(file_logger)
        // Global settings that apply to both Stdout logger and File logger
        .with(Targets::default().with_target(CRATE_ROOT, Level::TRACE))
        .init();

    Ok(())
}
