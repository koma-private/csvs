use crate::tui::tui_id::TuiId;
use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use tui_realm_stdlib::Phantom;
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::{Component, Event, MockComponent};

#[derive(MockComponent, Default)]
/// Listens for global input events in the TUI.
///
/// This component is invisible and processes keyboard inputs to trigger actions.
pub(crate) struct EventListener {
    /// Handles events without rendering any UI.
    component: Phantom,
}

impl Component<TuiMsg, TuiUserEvent> for EventListener {
    /// Maps keyboard events to TUI actions.
    ///
    /// # Arguments
    /// * `ev` - The input event to process.
    ///
    /// # Returns
    /// An optional `TuiMsg` for corresponding actions.
    fn on(&mut self, ev: Event<TuiUserEvent>) -> Option<TuiMsg> {
        match ev {
            // Close application on Ctrl+C.
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(TuiMsg::AppClose),

            // Show save dialog on Ctrl+S.
            Event::Keyboard(KeyEvent {
                code: Key::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(TuiMsg::SQLResultFilenameInputDialogShow),

            // Switch focus to specific components using function keys.
            Event::Keyboard(KeyEvent {
                code: Key::Function(number),
                modifiers: KeyModifiers::NONE,
            }) => {
                match number {
                    1 => Some(TuiMsg::MoveInputFocus(TuiId::AvailableTables)),
                    2 => Some(TuiMsg::MoveInputFocus(TuiId::SQLInput)),
                    3 => Some(TuiMsg::MoveInputFocus(TuiId::SQLResult)),
                    _ => {
                        None // Ignore unsupported function keys
                    }
                }
            }

            // Navigate focus forward or backward.
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(TuiMsg::ForwardInputFocus),
            Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => Some(TuiMsg::BackInputFocus),

            // Handle custom user events for dialogs and progress.
            Event::User(TuiUserEvent::MessageDialogShow(tui_id, dialog_type, message)) => {
                Some(TuiMsg::MessageDialogShow(tui_id, dialog_type, message))
            }
            Event::User(TuiUserEvent::ProgressDialogClose) => Some(TuiMsg::ProgressDialogClose),

            _ => {
                None // Ignore unhandled events.
            }
        }
    }
}
