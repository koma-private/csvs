use crate::tui::component_message_dialog::{ComponentMessageDialog, ComponentMessageDialogType};
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Displays a message dialog with the specified type and message.
    /// Mounts the dialog to the active view and updates the menu stack.
    ///
    /// # Arguments
    /// * `tui_id` - Identifier of the current menu.
    /// * `dialog_type` - Type of the dialog (e.g., Info or Error).
    /// * `message` - The content to display in the dialog.
    pub fn message_dialog_show(
        &mut self,
        tui_id: TuiId,
        dialog_type: ComponentMessageDialogType,
        message: String,
    ) -> Option<TuiMsg> {
        self.progress_dialog_close(); // Ensure no progress dialog is open.

        if !self.app.mounted(&TuiId::MessageDialog) {
            self.app
                .mount(
                    TuiId::MessageDialog,
                    Box::new(ComponentMessageDialog::new(dialog_type, message)),
                    Vec::new(),
                )
                .expect("Failed to mount message dialog");

            self.app
                .active(&TuiId::MessageDialog)
                .expect("Failed to activate message dialog");

            self.menu_stack.push(tui_id); // Save current menu to stack.

            Some(TuiMsg::ShortcutMenuUpdate(TuiId::MessageDialog))
        } else {
            None
        }
    }

    /// Closes the currently displayed message dialog.
    /// Unmounts the dialog and restores the previous menu from the stack.    
    pub fn message_dialog_close(&mut self) -> Option<TuiMsg> {
        if self.app.mounted(&TuiId::MessageDialog) {
            self.app
                .umount(&TuiId::MessageDialog)
                .expect("Failed to unmount message dialog");

            if let Some(tui_id) = self.menu_stack.pop() {
                Some(TuiMsg::ShortcutMenuUpdate(tui_id)) // Restore the previous menu.
            } else {
                Some(TuiMsg::UIElementDarken(false)) // Disable UI darkening if no menu remains.
            }
        } else {
            None
        }
    }
}
