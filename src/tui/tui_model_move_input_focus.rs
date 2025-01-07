use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::terminal::TerminalAdapter;
use tuirealm::Attribute;

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Moves the input focus to the specified TUI component.
    /// If the target element is not already focused, sets the focus to the target element.
    ///
    /// # Arguments
    /// * `tui_id` - Identifier of the component to focus.
    ///
    /// # Returns
    /// * `Some(TuiMsg)` to trigger a UI update, or `None` if no update is needed.
    pub fn move_input_focus(&mut self, tui_id: TuiId) -> Option<TuiMsg> {
        if !self.is_dialog_displayed() {
            let should_update = if let Ok(Some(focus)) = self.app.query(&tui_id, Attribute::Focus) {
                !focus.unwrap_flag() // Update only if the target element is not focused.
            } else {
                true
            };

            if should_update {
                self.app
                    .active(&tui_id)
                    .expect("Failed to activate the target component");
                Some(TuiMsg::ShortcutMenuUpdate(tui_id))
            } else {
                None
            }
        } else {
            None // Skip if a dialog is currently displayed.
        }
    }

    /// Moves the input focus forward to the next component in the flow.
    ///
    /// # Returns
    /// * `Some(TuiMsg)` to trigger a UI update, or `None` if no update is needed.
    pub fn forward_input_focus(&mut self) -> Option<TuiMsg> {
        match self.which_component_focused() {
            None => None,
            Some(focused) => self.move_input_focus(focused.move_forward()),
        }
    }

    /// Moves the input focus backward to the previous component in the flow.
    ///
    /// # Returns
    /// * `Some(TuiMsg)` to trigger a UI update, or `None` if no update is needed.
    pub fn back_input_focus(&mut self) -> Option<TuiMsg> {
        match self.which_component_focused() {
            None => None,
            Some(focused) => self.move_input_focus(focused.move_back()),
        }
    }
}
