use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use tui_realm_stdlib::Input;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Clear;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, State};

/// Filename input dialog for saving SQL results.
pub struct SQLResultFilenameInputDialog {
    /// Input field for capturing the filename.
    component: Input,
}

impl Default for SQLResultFilenameInputDialog {
    /// Initializes the dialog with default styles.
    fn default() -> Self {
        Self {
            component: Input::default()
                .title("Please input filename", Alignment::Left)
                .borders(Borders::default().modifiers(BorderType::Rounded))
                .input_type(InputType::Text)
                .inactive(Style::default().fg(Color::DarkGray)),
        }
    }
}

impl MockComponent for SQLResultFilenameInputDialog {
    /// Renders the dialog on the screen.
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

    /// Executes commands on the dialog.
    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<TuiMsg, TuiUserEvent> for SQLResultFilenameInputDialog {
    /// Handles events for user interaction.
    /// Processes filename input or closes the dialog.
    fn on(&mut self, ev: Event<TuiUserEvent>) -> Option<TuiMsg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Type(ch)),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                return Some(TuiMsg::SQLResultFilenameInputDialogClose);
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                let value = self.component.states.get_value();
                if !value.is_empty() {
                    return Some(TuiMsg::DatabaseRequestSaveResult(value));
                }
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        Some(TuiMsg::Redraw)
    }
}
