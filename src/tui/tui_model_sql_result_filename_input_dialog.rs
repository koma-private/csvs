use crate::tui::component_sql_result_filename_input_dialog::SQLResultFilenameInputDialog;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Displays the filename input dialog for saving SQL results.
    /// Mounts the dialog and saves the current menu state.
    ///
    /// # Returns
    /// * `Some(TuiMsg)` if the dialog is displayed, or `None` if it is already shown.
    pub fn sql_result_filename_input_dialog_show(&mut self) -> Option<TuiMsg> {
        if !self.app.mounted(&TuiId::SQLResultFilenameInputDialog) {
            self.app
                .mount(
                    TuiId::SQLResultFilenameInputDialog,
                    Box::new(SQLResultFilenameInputDialog::default()),
                    Vec::new(),
                )
                .expect("Failed to mount SQLResultFilenameInputDialog component");

            self.app
                .active(&TuiId::SQLResultFilenameInputDialog)
                .expect("Failed to activate SQLResultFilenameInputDialog component");

            self.menu_stack.push(TuiId::SQLResult);

            Some(TuiMsg::ShortcutMenuUpdate(
                TuiId::SQLResultFilenameInputDialog,
            ))
        } else {
            None
        }
    }

    /// Closes the filename input dialog and restores the previous menu.
    ///
    /// # Returns
    /// * `Some(TuiMsg)` to update the TUI state, or `None` if no dialog is displayed.
    pub fn sql_result_filename_input_dialog_close(&mut self) -> Option<TuiMsg> {
        if self.app.mounted(&TuiId::SQLResultFilenameInputDialog) {
            self.app
                .umount(&TuiId::SQLResultFilenameInputDialog)
                .expect("Failed to unmount SQLResultFilenameInputDialog component");

            if let Some(tui_id) = self.menu_stack.pop() {
                Some(TuiMsg::ShortcutMenuUpdate(tui_id))
            } else {
                Some(TuiMsg::UIElementDarken(false))
            }
        } else {
            None
        }
    }
}
