use crate::db::table_info::TableInfo;
use crate::tui::component_message_dialog::ComponentMessageDialogType;
use crate::tui::tui_id::TuiId;

/// Represents messages exchanged between components in the TUI.
#[derive(Debug, PartialEq, Clone)]
pub enum TuiMsg {
    /// Closes the application.
    AppClose,
    /// Indicates the selected table in the available tables list.
    AvailableTablesSelected(String),
    /// Show details of the selected table in the available tables list.
    DatabaseRequestTableInfo(String),
    TableInfoDialogClose,
    TableInfoDialogShow(String, Vec<TableInfo>),
    TableInfoColumnSelected(String),
    /// Closes the message dialog.
    MessageDialogClose,
    /// Displays a message dialog with a specific type and message.
    /// * `TuiId` - Identifier of the component initiating the dialog.
    /// * `ComponentMessageDialogType` - Type of the dialog (e.g., Info or Error).
    /// * `String` - The message content.
    MessageDialogShow(TuiId, ComponentMessageDialogType, String),
    /// Moves focus to a specific TUI component.
    MoveInputFocus(TuiId),
    /// Moves focus to the next TUI component.
    ForwardInputFocus,
    /// Moves focus to the previous TUI component.
    BackInputFocus,
    /// Updates the shortcut menu for a specific component.
    ShortcutMenuUpdate(TuiId),
    /// Requests a list of available database tables.
    DatabaseRequestAvailableTables,
    /// Sends a SQL query to the database.
    DatabaseRequestByQuery(String),
    /// Requests a specific page of SQL query results.
    /// * `usize` - Initial row position.
    /// * `usize` - Current page index.
    DatabaseRequestByPage(usize, usize),
    /// Requests to save SQL query results to a file.
    DatabaseRequestSaveResult(String),
    /// Updates the SQL input field with a new value.
    SQLInputValue(String),
    SQLInputAppendValue(String),
    /// Navigates forward in the SQL input history.
    SQLInputHistoryForward,
    /// Navigates backward in the SQL input history.
    SQLInputHistoryBack,
    /// Displays a progress dialog with a specific message.
    ProgressDialogShow(String),
    /// Closes the progress dialog.
    ProgressDialogClose,
    /// Closes the SQL result filename input dialog.
    SQLResultFilenameInputDialogClose,
    /// Displays the SQL result filename input dialog.
    SQLResultFilenameInputDialogShow,
    /// Toggles UI darkening.
    UIElementDarken(bool),
    /// Updates the SQL result information.
    UpdateSQLResultInfo(String),
    /// Forces a UI redraw.
    Redraw,
}
