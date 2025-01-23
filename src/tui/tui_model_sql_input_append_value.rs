use crate::tui::component_sql_input::ComponentSQLInput;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use std::ops::Add;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    pub fn sql_input_append_value(&mut self, value: String) -> Option<TuiMsg> {
        let state = self
            .app
            .state(&TuiId::SQLInput)
            .expect("Failed to get content of SQLInput component");

        let lines: Vec<String> = state
            .unwrap_vec()
            .iter()
            .map(|v| v.clone().unwrap_string())
            .collect();
        let lines = lines.join("\n").add(&value);

        self.sql_input_history_save(&lines); // Save the new value to history.

        // Update the SQL input field with the new value.
        self.app
            .umount(&TuiId::SQLInput)
            .expect("Failed to unmount SQLInput component");
        self.app
            .mount(
                TuiId::SQLInput,
                Box::new(ComponentSQLInput::new(&lines)),
                Vec::new(),
            )
            .expect("Failed to mount SQLInput component with new value");
        None
    }
}
