use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use crate::util::sqlite_quoted::SqliteQuoted;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Generates an SQL query to select all data from a table based on user selection.
    ///
    /// # Arguments
    /// * `selected` - Name of the selected table.
    ///
    /// # Returns
    /// * A `TuiMsg` containing the generated SQL query.
    pub fn available_tables_selected(&mut self, selected: String) -> Option<TuiMsg> {
        let table_name_quoted = SqliteQuoted::Field(selected.to_string()).get(); // Quote the table name for SQL safety
        let sql_query = format!("SELECT * FROM {};", table_name_quoted); // Construct SQL query
        Some(TuiMsg::SQLInputValue(sql_query)) // Return the query as a message
    }
}
