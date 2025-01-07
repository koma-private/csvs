use crate::db::statement_result::StatementPagedResult;
use crate::tui::component_message_dialog::ComponentMessageDialogType;
use crate::tui::tui_id::TuiId;

#[derive(Clone, Eq, PartialOrd, Debug)]
/// Represents custom user events for the TUI application.
pub enum TuiUserEvent {
    /// Executes a SQL query.
    RequestDatabaseByQuery(String),
    /// Fetches a specific page of results.
    /// * `usize` - Initial row position.
    /// * `usize` - Current page index.
    RequestDatabaseByPage(usize, usize),
    /// Sends a paginated result from the database back to the TUI.
    ResponseDatabasePagedResult(StatementPagedResult),
    /// Requests the list of available database tables.
    RequestDatabaseAvailableTables,
    /// Sends a list of available database tables back to the TUI.
    ResponseDatabaseAvailableTables(Vec<String>),
    /// Saves SQL query results to a specified file.
    RequestDatabaseSaveResult(String),
    /// Displays a message dialog.
    /// * `TuiId` - Identifier of the component initiating the dialog.
    /// * `ComponentMessageDialogType` - Type of the dialog (e.g., Info or Error).
    /// * `String` - The message content.
    MessageDialogShow(TuiId, ComponentMessageDialogType, String),
    /// Closes the progress dialog.
    ProgressDialogClose,
}

impl PartialEq<Self> for TuiUserEvent {
    /// Implements equality comparison for `TuiUserEvent` variants.
    fn eq(&self, other: &Self) -> bool {
        match self {
            TuiUserEvent::RequestDatabaseByQuery(_) => {
                matches!(other, TuiUserEvent::RequestDatabaseByQuery(_))
            }
            TuiUserEvent::RequestDatabaseByPage(_, _) => {
                matches!(other, TuiUserEvent::RequestDatabaseByPage(_, _))
            }
            TuiUserEvent::RequestDatabaseSaveResult(_) => {
                matches!(other, TuiUserEvent::RequestDatabaseSaveResult(_))
            }
            TuiUserEvent::ResponseDatabasePagedResult(_) => {
                matches!(other, TuiUserEvent::ResponseDatabasePagedResult(_))
            }
            TuiUserEvent::RequestDatabaseAvailableTables => {
                matches!(other, TuiUserEvent::RequestDatabaseAvailableTables)
            }
            TuiUserEvent::ResponseDatabaseAvailableTables(_) => {
                matches!(other, TuiUserEvent::ResponseDatabaseAvailableTables(_))
            }
            TuiUserEvent::MessageDialogShow(_, _, _) => {
                matches!(other, TuiUserEvent::MessageDialogShow(_, _, _))
            }
            TuiUserEvent::ProgressDialogClose => {
                matches!(other, TuiUserEvent::ProgressDialogClose)
            }
        }
    }
}
