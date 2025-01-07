use crate::app::app;
use crate::args::Args;
use crate::help::{safe_show_help, safe_show_version};
use crate::raw_args::RawArgs;
use clap::Parser;
use tracing::debug;

mod app;
mod args;
mod csv;
mod db;
mod format;
mod help;
mod logger;
mod quote_style;
mod raw_args;
mod sqlite_data_type;
mod trim;
mod tui;
mod util;

/// Application entry point
fn main() -> anyhow::Result<()> {
    // Parse command-line arguments
    let raw_args = RawArgs::try_parse()?;

    // Convert arguments into structured form
    let args: Args = raw_args.into();

    // Handle help or version requests
    if args.help {
        safe_show_help();
        return Ok(());
    }

    if args.version {
        safe_show_version();
        return Ok(());
    }

    // Initialize logging if specified
    if let Some(out_log) = args.out_log.clone() {
        logger::init_tracing(&out_log)?;
        debug!("Program started.");
    }

    // Execute main application logic
    app(args)?;

    debug!("Program completed successfully.");
    Ok(())
}
