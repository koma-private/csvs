use crate::db::list_available_table::list_available_tables;
use crate::tui::component_message_dialog::ComponentMessageDialogType;
use crate::tui::database_event_listener::DatabaseEventListener;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;
use tuirealm::Event;

impl DatabaseEventListener {
    /// Updates the list of available tables in the TUI.
    ///
    /// # Behavior
    /// - On success: Sends the list of tables to the TUI.
    /// - On error: Displays an error message dialog.
    pub fn available_tables(&mut self) -> Option<TuiMsg> {
        match list_available_tables(&self.pool) {
            Ok(available_tables) => {
                self.sender_user_event
                    .send(Event::User(TuiUserEvent::ResponseDatabaseAvailableTables(
                        available_tables.clone(),
                    )))
                    .expect("Failed to send available tables event");
                None
            }
            Err(err) => Some(TuiMsg::MessageDialogShow(
                TuiId::AvailableTables,
                ComponentMessageDialogType::Error,
                err.to_string(),
            )),
        }
    }
}
