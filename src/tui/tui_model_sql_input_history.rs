use crate::tui::component_sql_input::ComponentSQLInput;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Navigates forward in the SQL input history and updates the SQL input field.
    ///
    /// # Returns
    /// * `Some(TuiMsg)` if the input field is updated, or `None` otherwise.
    pub fn sql_input_history_forward(&mut self) -> Option<TuiMsg> {
        if let Some(recall) = self.input_history.forward() {
            self.app
                .umount(&TuiId::SQLInput)
                .expect("Failed to unmount SQLInput component");
            self.app
                .mount(
                    TuiId::SQLInput,
                    Box::new(ComponentSQLInput::new(recall)),
                    Vec::new(),
                )
                .expect("Failed to mount SQLInput component with forward history");
        }
        Some(TuiMsg::MoveInputFocus(TuiId::SQLInput))
    }

    /// Navigates backward in the SQL input history.
    /// Unmounts and remounts the SQL input component with the previous history entry.
    ///
    /// # Returns
    /// * `Some(TuiMsg)` to update the TUI state, or `None` if no update is needed.
    pub fn sql_input_history_back(&mut self) -> Option<TuiMsg> {
        if let Some(recall) = self.input_history.back() {
            self.app
                .umount(&TuiId::SQLInput)
                .expect("Failed to unmount SQLInput component");
            self.app
                .mount(
                    TuiId::SQLInput,
                    Box::new(ComponentSQLInput::new(recall)),
                    Vec::new(),
                )
                .expect("Failed to mount SQLInput component with backward history");
        }
        Some(TuiMsg::MoveInputFocus(TuiId::SQLInput))
    }

    /// Saves a new SQL input into the history.
    /// Avoids duplication and moves the cursor to the latest entry.
    ///
    /// # Arguments
    /// * `content` - The SQL input string to save in the history.
    pub fn sql_input_history_save(&mut self, content: &str) {
        if !self.input_history.contains(content) {
            self.input_history.push(content.to_string());
        } else {
            self.input_history.cursor_tail();
        }
    }
}
