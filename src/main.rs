use crate::app::app;
use args_util::args::Args;
use crate::help::{safe_show_help, safe_show_version};
use args_util::raw_args::RawArgs;
use clap::Parser;
use tracing::debug;

mod app;
mod csv;
mod db;
mod format;
mod help;
mod logger;
mod tui;
mod args_util;

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
