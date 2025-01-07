use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use tui_realm_stdlib::Paragraph;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, Borders, Color, TextSpan};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Clear;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, State};

/// Displays a progress dialog in the TUI.
pub struct ComponentProgressDialog {
    /// Paragraph component for rendering the dialog.
    component: Paragraph,
}

impl ComponentProgressDialog {
    /// Creates a new progress dialog with the specified message.
    ///
    /// # Arguments
    /// * `message` - Text to display in the dialog.
    pub fn new(message: String) -> Self {
        let spans: Vec<TextSpan> = message.split("\n").map(TextSpan::new).collect();
        Self {
            component: Paragraph::default()
                .borders(Borders::default().color(Color::Yellow))
                .title("Processing", Alignment::Left)
                .foreground(Color::Yellow)
                .text(&spans),
        }
    }
}

impl MockComponent for ComponentProgressDialog {
    /// Renders the dialog on the screen.
    /// Clears the area before rendering.
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area); // Clear the area before rendering
        self.component.view(frame, area)
    }

    /// Queries an attribute of the dialog.
    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    /// Sets an attribute for the dialog.
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

impl Component<TuiMsg, TuiUserEvent> for ComponentProgressDialog {
    /// Handles events for the dialog. Does nothing by default.
    fn on(&mut self, _: Event<TuiUserEvent>) -> Option<TuiMsg> {
        None
    }
}
