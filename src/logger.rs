use std::{fs::File, sync::Arc};

use tracing::Level;
use tracing_subscriber::{filter::Targets, prelude::*, Registry};

// TODO: Replace with `EnvFilter` approach using `RUST_LOG` environment variable

/// Initialize console and file tracing loggers at application startup.
///
/// Provides both per-layer and global filtering by leveraging `Targets`.
pub fn initialize_logger() -> std::io::Result<()> {
    let console_logger = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(Targets::new().with_default(Level::TRACE));

    let log_file = File::create("zinc.log")?;
    let file_logger = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(log_file));

    Registry::default()
        .with(console_logger)
        .with(file_logger)
        .with(
            Targets::default()
                .with_target("zinc", Level::TRACE)
                .with_target("tokio", Level::DEBUG),
        )
        .init();

    Ok(())
}
