use crate::db::sqlite_quoted::SqliteQuoted;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::terminal::TerminalAdapter;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    pub fn table_info_column_selected(&mut self, selected: String) -> Option<TuiMsg> {
        let column_name_quoted = SqliteQuoted::Field(selected.to_string()).get(); // Quote the table name for SQL safety
        let sql_query = format!(" {}", column_name_quoted); // Construct SQL query
        Some(TuiMsg::SQLInputAppendValue(sql_query)) // Return the query as a message
    }
}
