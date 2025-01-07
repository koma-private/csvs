use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use tuirealm::terminal::TerminalAdapter;
use tuirealm::Attribute;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Determines which component currently has focus.
    ///
    /// # Returns
    /// * `Some(TuiId)` if a component is focused, or `None` if no component is focused.
    pub fn which_component_focused(&self) -> Option<TuiId> {
        let components = vec![TuiId::AvailableTables, TuiId::SQLInput, TuiId::SQLResult];

        components.into_iter().find(|&tui_id| {
            self.app
                .query(&tui_id, Attribute::Focus)
                .map_or(false, |attr| attr.map_or(false, |v| v.unwrap_flag()))
        })
    }
}
