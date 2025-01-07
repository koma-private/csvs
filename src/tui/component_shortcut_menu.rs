use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use tui_realm_stdlib::Span;
use tuirealm::props::{Color, TextSpan};
use tuirealm::{Component, Event, MockComponent};

#[derive(MockComponent)]
/// Displays a menu of keyboard shortcuts in the TUI.
pub(crate) struct ComponentShortcutMenu {
    /// Span component used to render the menu.
    component: Span,
}

impl Default for ComponentShortcutMenu {
    /// Creates a default shortcut menu.
    fn default() -> Self {
        Self {
            component: Span::default().foreground(Color::Reset),
        }
    }
}

impl Component<TuiMsg, TuiUserEvent> for ComponentShortcutMenu {
    /// Currently, this component does not handle events.
    fn on(&mut self, _: Event<TuiUserEvent>) -> Option<TuiMsg> {
        None
    }
}

#[derive(Clone, Debug)]
/// Represents a single shortcut with a key combination and description.
pub(crate) struct ShortCut {
    /// Key combination for the shortcut.
    pub label: String,
    /// Description of the shortcut's action.
    pub desc: String,
}

impl ShortCut {
    /// Creates a new shortcut.
    pub fn new(label: &str, desc: &str) -> Self {
        Self {
            label: label.to_string(),
            desc: desc.to_string(),
        }
    }

    /// Formats the shortcut for rendering as text spans.
    pub fn render_to_spans(&self) -> Vec<TextSpan> {
        vec![
            self.label.to_string().into(),
            " ".into(), // Add a space between label and description
            TextSpan::new(self.desc.clone()).bg(Color::Blue),
        ]
    }
}

#[derive(Clone, Debug)]
/// Represents a collection of shortcuts displayed as a menu.
pub(crate) struct ShortCutMenu {
    /// List of shortcuts in the menu.
    pub menu: Vec<ShortCut>,
}

impl ShortCutMenu {
    /// Creates a new shortcut menu.
    pub fn new(menu: Vec<ShortCut>) -> Self {
        Self { menu }
    }

    /// Formats the shortcut menu for rendering as text spans.
    pub fn render_to_spans(&self) -> Vec<TextSpan> {
        let spans: Vec<Vec<TextSpan>> = self.menu.iter().map(|v| v.render_to_spans()).collect();

        let space = TextSpan::new("  ");
        let line: Vec<TextSpan> = spans.join(&space);
        line
    }
}
