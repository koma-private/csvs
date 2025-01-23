use crate::db::table_info::TableInfo;
use crate::tui::component_table_info_dialog::ComponentTableInfoDialog;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    pub fn table_info_dialog_show(
        &mut self,
        table_name: String,
        table_infos: Vec<TableInfo>,
    ) -> Option<TuiMsg> {
        self.progress_dialog_close(); // Ensure no progress dialog is open.

        if !self.app.mounted(&TuiId::TableInfoDialog) {
            self.app
                .mount(
                    TuiId::TableInfoDialog,
                    Box::new(ComponentTableInfoDialog::new(&table_name, table_infos)),
                    Vec::new(),
                )
                .expect("Failed to mount table info dialog");

            self.app
                .active(&TuiId::TableInfoDialog)
                .expect("Failed to activate table info dialog");

            self.menu_stack.push(TuiId::AvailableTables); // Save current menu to stack.

            Some(TuiMsg::ShortcutMenuUpdate(TuiId::TableInfoDialog))
        } else {
            None
        }
    }

    /// Closes the currently displayed table info dialog.
    /// Unmounts the dialog and restores the previous menu from the stack.    
    pub fn table_info_dialog_close(&mut self) -> Option<TuiMsg> {
        if self.app.mounted(&TuiId::TableInfoDialog) {
            self.app
                .umount(&TuiId::TableInfoDialog)
                .expect("Failed to unmount table info dialog");

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
