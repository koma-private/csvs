use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use tui_realm_stdlib::Span;
use tuirealm::props::Color;
use tuirealm::{Component, Event, MockComponent};

#[derive(MockComponent)]
/// Displays summary information about SQL results in the TUI.
pub(crate) struct ComponentSQLResultInfo {
    /// Span component to render result details.
    component: Span,
}

impl Default for ComponentSQLResultInfo {
    /// Creates a default instance.
    fn default() -> Self {
        Self {
            component: Span::default().foreground(Color::Reset),
        }
    }
}

impl Component<TuiMsg, TuiUserEvent> for ComponentSQLResultInfo {
    /// This component does not handle any events.
    fn on(&mut self, _: Event<TuiUserEvent>) -> Option<TuiMsg> {
        None
    }
}
