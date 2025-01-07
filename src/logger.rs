use std::fs::File;
use tracing::Level;
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initializes tracing to log debug and above levels to a file.
///
/// # Arguments
/// * `log_file_path` - Path to the log file.
///
/// # Returns
/// Error if the log file cannot be created.
pub fn init_tracing(log_file_path: &str) -> anyhow::Result<()> {
    let log_file = File::create(log_file_path)?; // Create log file
    let file_layer = tracing_logfmt::builder().layer().with_writer(log_file); // Configure log output
    let filter = filter::Targets::new().with_target("csvs", Level::DEBUG); // Set log level

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .init(); // Initialize subscriber

    Ok(())
}
