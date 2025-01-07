use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Checks if any dialog is currently displayed.
    ///
    /// # Returns
    /// * `true` if a dialog (e.g., message or filename input) is mounted.
    /// * `false` otherwise.
    pub fn is_dialog_displayed(&self) -> bool {
        self.app.mounted(&TuiId::MessageDialog)
            || self.app.mounted(&TuiId::SQLResultFilenameInputDialog)
    }
}
