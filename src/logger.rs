//! Logging and telemetry module for storing, formatting and displaying events.

use std::{fs::File, sync::Arc};

use tracing::Level;
use tracing_subscriber::{filter::Targets, prelude::*, Registry};

/// Initialize and configure console and file-based logging.
///
/// Provides per-layer and global filtering by leveraging `Targets`. Spans and
/// events are emitted to the console or written to a file depending on their
/// severity.
pub fn initialize_logger() -> std::io::Result<()> {
    // The console layer emits events to stdout and are intended to be human-readable
    let console_logger = tracing_subscriber::fmt::layer().pretty().with_ansi(true);

    // File logger layer used to store persistent log data;
    let filepath = File::create("zinc.log").expect("failed to create logging output file");
    let file_logger = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(filepath));

    // let env_filter = set_env_filter();
    let global_filter = Targets::default()
        .with_target("zinc", Level::TRACE)
        .with_target("tokio", Level::WARN);

    Registry::default()
        // Log `INFO` level and above to `stdout`
        .with(console_logger.with_filter(Targets::default().with_default(Level::INFO)))
        .with(file_logger)
        .with(global_filter)
        .init();

    Ok(())
}

// fn set_env_filter() -> Result<EnvFilter, FromEnvError> {
//     match EnvFilter::try_from_default_env() {
//         Ok(filter) => {
//             println!("[set_env_filter] => {}", filter.to_string());
//             Ok(filter)
//         }
//         Err(err) => {
//             eprintln!("[set_env_filter]: {:?}", err);
//             Err(err)
//         }
//     }
// }
