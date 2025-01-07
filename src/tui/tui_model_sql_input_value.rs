use crate::tui::component_sql_input::ComponentSQLInput;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Updates the value of the SQL input field in the TUI.
    /// Saves the new value to the input history and sets it in the SQL input field.
    ///
    /// # Arguments
    /// * `value` - The new input value to be displayed in the SQL input field.
    pub fn sql_input_value(&mut self, value: String) -> Option<TuiMsg> {
        self.sql_input_history_save(&value); // Save the new value to history.

        // Update the SQL input field with the new value.
        self.app
            .umount(&TuiId::SQLInput)
            .expect("Failed to unmount SQLInput component");
        self.app
            .mount(
                TuiId::SQLInput,
                Box::new(ComponentSQLInput::new(&value)),
                Vec::new(),
            )
            .expect("Failed to mount SQLInput component with new value");
        None
    }
}
