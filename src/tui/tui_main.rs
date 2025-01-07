use crate::args::Args;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;

use tracing::debug;
use tuirealm::{PollStrategy, Update};

/// Entry point for the TUI application.
/// Initializes the TUI model and manages the main application loop.
///
/// # Arguments
/// * `pool` - SQLite connection pool.
/// * `args` - Command-line arguments for the application.
///
/// # Returns
/// * `Ok(())` if the application runs and exits successfully.
/// * An error if initialization or runtime operations fail.
pub fn tui_main(pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, args: Args) -> anyhow::Result<()> {
    // Initialize the TUI model with database connection and command-line arguments.
    let mut model = TuiModel::new(pool, args)?;

    // Preload available tables into the shortcut menu and database view.
    model.update(Some(TuiMsg::ShortcutMenuUpdate(TuiId::AvailableTables)));
    model.update(Some(TuiMsg::DatabaseRequestAvailableTables));

    debug!("Starting TUI main loop.");

    // Main loop for the TUI application.
    while !model.should_quit {
        // Process a tick of the TUI application to handle events and retrieve messages.
        let messages = model.app.tick(PollStrategy::Once)?;
        if !messages.is_empty() {
            model.should_redraw = true; // Flag for redraw if any messages are processed.
            for msg in messages.into_iter() {
                let mut msg = Some(msg);
                while msg.is_some() {
                    msg = model.update(msg);
                }
            }
        }

        // Redraw the TUI interface if necessary.
        if model.should_redraw {
            model.view(); // Render the UI based on the current model state.
            model.should_redraw = false;
        }
    }

    // Restore terminal settings before exiting.
    model.restore_terminal()?;
    Ok(())
}
