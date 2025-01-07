use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::props::{Color, PropPayload, PropValue, TextSpan};
use tuirealm::terminal::TerminalAdapter;
use tuirealm::{AttrValue, Attribute};

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Updates the SQL result info component with new details.
    /// This updates the result info table displayed in the TUI.
    ///
    /// # Arguments
    /// * `info` - A vector of tuples containing information to update.
    pub fn update_sql_result_info(&mut self, info: String) -> Option<TuiMsg> {
        let mut span = TextSpan::new(info);
        span.fg = Color::Gray;
        span.bg = Color::DarkGray;

        self.app
            .attr(
                &TuiId::SQLResultInfo,
                Attribute::Text,
                AttrValue::Payload(PropPayload::Vec(vec![PropValue::TextSpan(span)])),
            )
            .expect("Failed to update SQLResultInfo component");

        None
    }
}
