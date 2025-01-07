use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::props::Color;
use tuirealm::terminal::TerminalAdapter;
use tuirealm::{AttrValue, Attribute};

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Toggles the UI elements between darkened and normal states.
    /// The `flag` determines whether to apply or remove the darkened effect.
    ///
    /// # Arguments
    /// * `flag` - A boolean indicating whether to darken (true) or reset (false) UI elements.
    ///
    /// # Returns
    /// * Always returns `None`.
    pub fn ui_elements_darken(&mut self, flag: bool) -> Option<TuiMsg> {
        let color = if flag { Color::DarkGray } else { Color::Reset };
        let highlighted_color = if flag { Color::DarkGray } else { Color::Yellow };

        self.app
            .attr(
                &TuiId::AvailableTables,
                Attribute::Foreground,
                AttrValue::Color(color),
            )
            .expect("Failed to update foreground color for AvailableTables component");

        self.app
            .attr(
                &TuiId::AvailableTables,
                Attribute::HighlightedColor,
                AttrValue::Color(highlighted_color),
            )
            .expect("Failed to update highlighted color for AvailableTables component");

        self.app
            .attr(
                &TuiId::SQLResult,
                Attribute::Foreground,
                AttrValue::Color(color),
            )
            .expect("Failed to update foreground color for SQLResult component");

        self.app
            .attr(
                &TuiId::SQLResult,
                Attribute::HighlightedColor,
                AttrValue::Color(highlighted_color),
            )
            .expect("Failed to update highlighted color for SQLResult component");

        None
    }
}
