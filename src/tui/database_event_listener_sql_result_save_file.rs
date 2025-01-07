use crate::csv::csv_writer::CsvWriter;
use crate::tui::component_message_dialog::ComponentMessageDialogType;
use crate::tui::database_event_listener::{DatabaseEventListener, IS_SAVING, QUERY_RESULT};
use crate::tui::tui_id::TuiId;
use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;
use tuirealm::Event;

impl DatabaseEventListener {
    /// Saves the SQL query result to a specified file.
    ///
    /// # Behavior
    /// - Determines file format (TSV/CSV) based on the extension.
    /// - Displays a progress dialog during the save operation.
    /// - Shows success or error messages based on the outcome.
    ///
    /// # Arguments
    /// * `filename` - The path to save the query result.
    pub fn sql_result_save_file(&mut self, filename: String) -> Option<TuiMsg> {
        {
            let is_saving = IS_SAVING.read().unwrap();
            if *is_saving {
                return None; // Prevent concurrent save operations
            }
        }

        let mut args = self.args.clone();
        args.out_file = Some(filename.clone());
        let sender_user_event = self.sender_user_event.clone();

        std::thread::spawn(move || {
            {
                let mut is_saving = IS_SAVING.write().unwrap();
                *is_saving = true;
            }

            let query_result_read = QUERY_RESULT.read().unwrap();
            if query_result_read.rows.is_empty() {
                sender_user_event
                    .send(Event::User(TuiUserEvent::MessageDialogShow(
                        TuiId::SQLResult,
                        ComponentMessageDialogType::Error,
                        "SQL Result is empty and cannot be written to a file.".to_string(),
                    )))
                    .expect("Failed to send empty result message");
            } else {
                match CsvWriter::new(&args) {
                    Ok(mut writer) => {
                        if !args.out_without_header {
                            writer
                                .write_record(&query_result_read.header)
                                .expect("Failed to write headers");
                        }
                        writer
                            .write_records(&query_result_read.rows)
                            .expect("Failed to write rows");
                        writer.flush().expect("Failed to flush writer");

                        sender_user_event
                            .send(Event::User(TuiUserEvent::MessageDialogShow(
                                TuiId::SQLResult,
                                ComponentMessageDialogType::Info,
                                format!("File saved: {}", args.out_file.unwrap()),
                            )))
                            .expect("Failed to send success message");
                    }
                    Err(err) => {
                        sender_user_event
                            .send(Event::User(TuiUserEvent::MessageDialogShow(
                                TuiId::SQLResult,
                                ComponentMessageDialogType::Error,
                                format!("Failed to save file: {}", err),
                            )))
                            .expect("Failed to send error message");
                    }
                }
            }

            {
                let mut is_saving = IS_SAVING.write().unwrap();
                *is_saving = false;
            }
        });
        Some(TuiMsg::ProgressDialogShow(format!(
            "Saving {}...",
            filename
        )))
    }
}
