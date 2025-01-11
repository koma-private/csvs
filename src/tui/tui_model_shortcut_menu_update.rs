use crate::tui::component_shortcut_menu::{ShortCut, ShortCutMenu};
use crate::tui::tui_id::TuiId;
use crate::tui::tui_model::TuiModel;
use crate::tui::tui_msg::TuiMsg;
use tuirealm::props::{PropPayload, PropValue};
use tuirealm::terminal::TerminalAdapter;
use tuirealm::{AttrValue, Attribute};

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Updates the shortcut menu in the TUI based on the active component ID.
    /// Displays context-specific shortcuts for navigation and actions.
    ///
    /// # Arguments
    /// * `tui_id` - Identifier of the active TUI component.
    ///
    /// # Returns
    /// * `Some(TuiMsg::UIElementDarken)` indicating whether the UI should darken.
    pub fn shortcut_menu_update(&mut self, tui_id: TuiId) -> Option<TuiMsg> {
        // Common shortcuts available across all components
        let common_shortcuts = Vec::from([
            ShortCut::new("Tab/BackTab/F1-3", "Move Input Focus"),
            ShortCut::new("CTRL+C", "Quit"),
            ShortCut::new("CTRL+S", "Save Result"),
        ]);

        // Generate spans based on the active TUI ID
        let spans = match tui_id {
            TuiId::SQLInput => {
                let execute_sql = if self.keyboard_enhancement_enabled {
                    ShortCut::new("SHIFT+Enter", "Execute SQL")
                } else {
                    ShortCut::new("CTRL+X", "Execute SQL")
                };

                let mut temp = Vec::from([
                    execute_sql,
                    ShortCut::new("←→↑↓", "Move Cursor"),
                    ShortCut::new("CTRL+↑↓", "Input History"),
                    ShortCut::new("Home", "To Top"),
                    ShortCut::new("End", "To Bottom"),
                    ShortCut::new("PageUp", "Scroll↑"),
                    ShortCut::new("PageDown", "Scroll↓"),
                    ShortCut::new("CTRL+Z", "Undo"),
                    ShortCut::new("CTRL+Y", "Redo"),
                    ShortCut::new("CTRL+V", "Paste"),
                ]);

                if !self.is_dialog_displayed() {
                    temp = [&common_shortcuts[..], &temp[..]].concat();
                }
                Some(ShortCutMenu::new(temp).render_to_spans())
            }
            TuiId::AvailableTables => {
                let mut temp = Vec::from([
                    ShortCut::new("Enter", "Select Table"),
                    ShortCut::new("CTRL+I", "Table Info"),
                    ShortCut::new("↑↓", "Move Cursor"),
                    ShortCut::new("Home", "To Top"),
                    ShortCut::new("End", "To Bottom"),
                    ShortCut::new("PageUp", "Scroll↑"),
                    ShortCut::new("PageDown", "Scroll↓"),
                ]);

                if !self.is_dialog_displayed() {
                    temp = [&common_shortcuts[..], &temp[..]].concat();
                }
                Some(ShortCutMenu::new(temp).render_to_spans())
            }
            TuiId::SQLResult => {
                let mut temp = Vec::from([
                    ShortCut::new("↑↓", "Move Cursor"),
                    ShortCut::new("Home", "To Begin"),
                    ShortCut::new("End", "To End"),
                    ShortCut::new("PageUp", "Scroll↑"),
                    ShortCut::new("PageDown", "Scroll↓"),
                ]);

                if !self.is_dialog_displayed() {
                    temp = [&common_shortcuts[..], &temp[..]].concat();
                }
                Some(ShortCutMenu::new(temp).render_to_spans())
            }
            TuiId::MessageDialog => Some(
                ShortCutMenu::new(Vec::from([ShortCut::new("Any", "Close message")]))
                    .render_to_spans(),
            ),
            TuiId::SQLResultFilenameInputDialog => Some(
                ShortCutMenu::new(Vec::from([
                    ShortCut::new("Esc", "Cancel"),
                    ShortCut::new("Enter", "Write File"),
                    ShortCut::new("←→", "Move Cursor"),
                    ShortCut::new("Home", "To Begin"),
                    ShortCut::new("End", "To End"),
                ]))
                .render_to_spans(),
            ),
            TuiId::TableInfoDialog => Some(
                ShortCutMenu::new(Vec::from([
                    ShortCut::new("Esc", "Cancel"),
                    ShortCut::new("Enter", "Input Column Name"),
                    ShortCut::new("↑↓", "Move Cursor"),
                    ShortCut::new("Home", "To Begin"),
                    ShortCut::new("End", "To End"),
                    ShortCut::new("PageUp", "Scroll↑"),
                    ShortCut::new("PageDown", "Scroll↓"),
                ]))
                .render_to_spans(),
            ),
            _ => None,
        };

        // Apply the generated spans to the shortcut menu component
        match spans {
            None => {
                self.app
                    .attr(
                        &TuiId::ShortcutMenu,
                        Attribute::Text,
                        AttrValue::Payload(PropPayload::Vec(vec![])),
                    )
                    .expect("Failed to update shortcut menu");
            }
            Some(spans) => {
                let spans: Vec<PropValue> = spans
                    .iter()
                    .map(|v| PropValue::TextSpan(v.clone()))
                    .collect();
                self.app
                    .attr(
                        &TuiId::ShortcutMenu,
                        Attribute::Text,
                        AttrValue::Payload(PropPayload::Vec(spans)),
                    )
                    .expect("Failed to update shortcut menu");
            }
        }

        // Update the UI darken state based on the TUI ID
        match tui_id {
            TuiId::SQLResultFilenameInputDialog | TuiId::MessageDialog => {
                Some(TuiMsg::UIElementDarken(true))
            }
            _ => Some(TuiMsg::UIElementDarken(false)),
        }
    }
}
