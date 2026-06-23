use tracing_subscriber::{layer::{Layer, SubscriberExt}, util::SubscriberInitExt, EnvFilter};

/// Initialize tracing with stdout logging.
///
/// Set `OTLP_ENDPOINT` env var to enable OTLP export (not yet implemented).
/// Falls back to stdout logging with the configured filter.
pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true);

    tracing_subscriber::registry()
        .with(fmt_layer.with_filter(env_filter))
        .init();

    Ok(())
}

/// Initialize tracing for tests (no OTLP, just test writer).
#[cfg(test)]
pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
}
