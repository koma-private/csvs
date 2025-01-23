use crate::db::table_info::table_info;
use crate::tui::component_message_dialog::ComponentMessageDialogType;
use crate::tui::database_event_listener::DatabaseEventListener;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_msg::TuiMsg;

impl DatabaseEventListener {
    /// Updates the list of available tables in the TUI.
    ///
    /// # Behavior
    /// - On success: Sends the list of tables to the TUI.
    /// - On error: Displays an error message dialog.
    pub fn table_info(&mut self, table_name: &str) -> Option<TuiMsg> {
        match table_info(&self.pool, table_name) {
            Ok(table_infos) => {
                let table_infos_filtered = table_infos
                    .iter()
                    .filter(|v| v.name.ne(&self.args.raw_id))
                    .cloned()
                    .collect();

                Some(TuiMsg::TableInfoDialogShow(
                    table_name.to_string(),
                    table_infos_filtered,
                ))
            }
            Err(err) => Some(TuiMsg::MessageDialogShow(
                TuiId::AvailableTables,
                ComponentMessageDialogType::Error,
                err.to_string(),
            )),
        }
    }
}
