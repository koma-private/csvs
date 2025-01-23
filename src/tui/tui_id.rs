/// Identifiers for different components in the TUI.
/// Each variant represents a unique part of the TUI that can gain focus or interact with the user.
#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy, PartialOrd)]
pub enum TuiId {
    /// Focuses the table view displaying available database tables.
    AvailableTables,
    /// Handles user notifications or error messages.
    MessageDialog,
    /// Displays progress during operations.
    ProgressDialog,
    /// Processes input events without a visible UI component.
    KeyEventListener,
    /// Manages database-related events.
    DatabaseEventListener,
    /// Focuses the input field for SQL queries.
    SQLInput,
    /// Displays results of SQL queries.
    SQLResult,
    /// Shows metadata or supplementary details about SQL query results.
    SQLResultInfo,
    /// Handles the dialog for specifying filenames for exporting SQL results.
    SQLResultFilenameInputDialog,
    /// Displays the shortcut menu for quick access to actions.
    ShortcutMenu,
    TableInfoDialog,
}

impl TuiId {
    /// Updates the focus to the next UI component in the TUI flow.
    /// For example, navigating from AvailableTables to SQLInput.
    /// Returns `*self` if no focus change is required.
    pub fn move_forward(&self) -> Self {
        match self {
            TuiId::AvailableTables => Self::SQLInput,
            TuiId::SQLInput => Self::SQLResult,
            TuiId::SQLResult => Self::AvailableTables,
            _ => *self,
        }
    }

    /// Updates the focus to the previous UI component in the TUI flow.
    /// For example, navigating from SQLInput back to AvailableTables.
    /// Returns `*self` if no focus change is required.
    pub fn move_back(&self) -> Self {
        match self {
            TuiId::AvailableTables => Self::SQLResult,
            TuiId::SQLInput => Self::AvailableTables,
            TuiId::SQLResult => Self::SQLInput,
            _ => *self,
        }
    }
}
