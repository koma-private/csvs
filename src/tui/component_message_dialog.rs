use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;
use std::fmt::{Display, Formatter};

use tui_realm_stdlib::Paragraph;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, Borders, Color, TextSpan};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Clear;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, State};

/// Displays message dialogs in the TUI.
pub struct ComponentMessageDialog {
    /// Paragraph component for rendering dialog text.
    component: Paragraph,
}

/// Supported message dialog types.
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd)]
pub enum ComponentMessageDialogType {
    /// Informational dialog.
    Info,
    /// Error dialog.
    Error,
}

impl Display for ComponentMessageDialogType {
    /// Converts dialog type to a string (e.g., "INFO", "ERROR").
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentMessageDialogType::Info => f.write_str("INFO"),
            ComponentMessageDialogType::Error => f.write_str("ERROR"),
        }
    }
}

impl ComponentMessageDialog {
    /// Creates a new dialog with a specified type and message.
    ///
    /// # Arguments
    /// * `dialog_type` - Type of the dialog (`Info` or `Error`).
    /// * `message` - Text content of the dialog.
    pub(crate) fn new(dialog_type: ComponentMessageDialogType, message: String) -> Self {
        let spans: Vec<TextSpan> = message.split("\n").map(TextSpan::new).collect();
        let color = match dialog_type {
            ComponentMessageDialogType::Info => Color::Green,
            ComponentMessageDialogType::Error => Color::Red,
        };

        Self {
            component: Paragraph::default()
                .borders(Borders::default().color(color))
                .title(dialog_type.to_string(), Alignment::Left)
                .foreground(color)
                .text(&spans),
        }
    }
}

impl MockComponent for ComponentMessageDialog {
    /// Renders the dialog on the screen.
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area); // Clear the area before rendering
        self.component.view(frame, area)
    }

    /// Queries an attribute of the dialog.
    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    /// Sets an attribute of the dialog.
    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value)
    }

    /// Retrieves the dialog's state.
    fn state(&self) -> State {
        self.component.state()
    }

    /// Executes a command on the dialog.
    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<TuiMsg, TuiUserEvent> for ComponentMessageDialog {
    /// Handles events for the dialog.
    /// Closes the dialog on any keyboard event.
    fn on(&mut self, ev: Event<TuiUserEvent>) -> Option<TuiMsg> {
        match ev {
            Event::Keyboard(_) => Some(TuiMsg::MessageDialogClose),
            _ => None,
        }
    }
}
