use crate::db::execute_statements::execute_statements;
use crate::tui::component_message_dialog::ComponentMessageDialogType;
use crate::tui::database_event_listener::{DatabaseEventListener, IS_QUERYING, QUERY_RESULT};
use crate::tui::tui_id::TuiId;
use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;
use tuirealm::Event;

impl DatabaseEventListener {
    /// Executes an SQL query and updates the TUI with the results.
    ///
    /// # Arguments
    /// * `query` - The SQL query to execute.
    ///
    /// # Returns
    /// - Shows progress during execution.
    /// - Updates components with results or error messages.
    pub fn sql_execute(&mut self, query: String) -> Option<TuiMsg> {
        {
            let is_querying = IS_QUERYING.read().unwrap();
            if *is_querying {
                return None; // Prevent concurrent queries.
            }
        }

        let dialect = sqlparser::dialect::SQLiteDialect {};

        match sqlparser::parser::Parser::parse_sql(&dialect, &query) {
            Ok(statements) => {
                {
                    let mut is_querying = IS_QUERYING.write().unwrap();
                    *is_querying = true;
                }

                let pool = self.pool.clone();
                let raw_id = self.args.raw_id.clone();
                let sender_user_event = self.sender_user_event.clone();
                std::thread::spawn(move || {
                    match execute_statements(&pool, statements, Some(raw_id)) {
                        Ok(result) => {
                            let last = result.last().unwrap();
                            {
                                let mut query_result_write = QUERY_RESULT.write().unwrap();
                                *query_result_write = last.clone();
                            }
                            let paged_result = Self::sql_result_by_page(last, 0, 0).unwrap();

                            // Close progress dialog and update TUI components.
                            sender_user_event
                                .send(Event::User(TuiUserEvent::ProgressDialogClose))
                                .expect("Failed to close progress dialog");

                            sender_user_event
                                .send(Event::User(TuiUserEvent::ResponseDatabasePagedResult(
                                    paged_result,
                                )))
                                .expect("Failed to send paged result");

                            sender_user_event
                                .send(Event::User(TuiUserEvent::RequestDatabaseAvailableTables))
                                .expect("Failed to request available tables");
                        }
                        Err(err) => {
                            sender_user_event
                                .send(Event::User(TuiUserEvent::MessageDialogShow(
                                    TuiId::SQLInput,
                                    ComponentMessageDialogType::Error,
                                    err.to_string(),
                                )))
                                .expect("Failed to display error message");
                        }
                    }
                    {
                        let mut is_querying = IS_QUERYING.write().unwrap();
                        *is_querying = false;
                    }
                });
                Some(TuiMsg::ProgressDialogShow(format!(
                    "Executing {}...",
                    query
                )))
            }
            Err(err) => Some(TuiMsg::MessageDialogShow(
                TuiId::SQLInput,
                ComponentMessageDialogType::Error,
                err.to_string(),
            )),
        }
    }
}
