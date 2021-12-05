use tracing_subscriber::prelude::*;
use tracing_subscriber::util::TryInitError;
use tracing_subscriber::{fmt, fmt::format::FmtSpan, EnvFilter};

// TODO: Cleanup logic and fix issues with directives
pub fn init_tracing(directives: &str) -> Result<(), TryInitError> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(directives));

    tracing::debug!("Setting subscriber; directives: {}", directives);

    let format_layer = fmt::layer().with_span_events(FmtSpan::FULL);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(format_layer)
        .try_init()?;

    Ok(())
}
