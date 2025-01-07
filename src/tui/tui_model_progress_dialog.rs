use crate::tui::component_progress_dialog::ComponentProgressDialog;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Displays the progress dialog with a given message.
    /// Mounts the dialog to the TUI and darkens other UI elements.
    ///
    /// # Arguments
    /// * `value` - Message to display in the progress dialog.
    ///
    /// # Returns
    /// * `Some(TuiMsg::UIElementDarken(true))` if the dialog is displayed.
    /// * `None` if the dialog is already displayed.
    pub fn progress_dialog_show(&mut self, value: String) -> Option<TuiMsg> {
        if !self.app.mounted(&TuiId::ProgressDialog) {
            self.app
                .mount(
                    TuiId::ProgressDialog,
                    Box::new(ComponentProgressDialog::new(value)),
                    Vec::new(),
                )
                .expect("Failed to mount progress dialog");

            Some(TuiMsg::UIElementDarken(true))
        } else {
            None
        }
    }

    /// Closes the currently displayed progress dialog.
    /// Unmounts the dialog and restores normal UI brightness.
    ///
    /// # Returns
    /// * `Some(TuiMsg::UIElementDarken(false))` if the dialog is closed.
    /// * `None` if no dialog is displayed.
    pub fn progress_dialog_close(&mut self) -> Option<TuiMsg> {
        if self.app.mounted(&TuiId::ProgressDialog) {
            self.app
                .umount(&TuiId::ProgressDialog)
                .expect("Failed to unmount progress dialog");
            Some(TuiMsg::UIElementDarken(false))
        } else {
            None
        }
    }
}
