use crate::app::app;
use crate::help::{safe_show_help, safe_show_version};
use args_util::args::Args;
use args_util::raw_args::RawArgs;
use clap::Parser;
use tracing::debug;

mod app;
mod args_util;
mod csv;
mod db;
mod format;
mod help;
mod logger;
mod tui;

/// Application entry point
fn main() -> anyhow::Result<()> {
    // Parse command-line arguments
    match get_args() {
        Ok(args) => {
            // Handle help or version requests
            if args.help {
                safe_show_help();
            } else if args.version {
                safe_show_version();
            } else {
                // Initialize logging if specified
                if let Some(out_log) = &args.out_log {
                    logger::init_tracing(out_log)?;
                    debug!("Program started.");
                }

                // Execute main application logic
                app(args)?;
            }
            debug!("Program completed successfully.");
            Ok(())
        }
        Err(err) => {
            eprintln!("{}", err);
            safe_show_help();
            Err(anyhow::anyhow!(err))
        }
    }
}

fn get_args() -> anyhow::Result<Args> {
    let args: Args = RawArgs::try_parse()?.try_into()?;
    Ok(args)
}
