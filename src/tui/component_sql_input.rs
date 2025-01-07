use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use tui_realm_textarea::{
    TextArea, TEXTAREA_CMD_MOVE_BOTTOM, TEXTAREA_CMD_MOVE_PARAGRAPH_BACK,
    TEXTAREA_CMD_MOVE_PARAGRAPH_FORWARD, TEXTAREA_CMD_MOVE_TOP, TEXTAREA_CMD_MOVE_WORD_BACK,
    TEXTAREA_CMD_MOVE_WORD_FORWARD, TEXTAREA_CMD_NEWLINE, TEXTAREA_CMD_PASTE, TEXTAREA_CMD_REDO,
    TEXTAREA_CMD_UNDO,
};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, Style, TextModifiers};
use tuirealm::ratatui::layout::Rect;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, State};

/// TUI component for SQL input with history support.
pub struct ComponentSQLInput<'a> {
    /// TextArea for receiving SQL commands.
    component: TextArea<'a>,
}

impl ComponentSQLInput<'_> {
    /// Creates a new SQL input component.
    ///
    /// # Arguments
    /// * `content` - Initial content of the SQL input area.
    pub(crate) fn new(content: &str) -> Self {
        let content: Vec<String> = content.split("\n").map(|v| v.to_string()).collect();
        Self {
            component: TextArea::new(content)
                .title("SQL Input (F2)", Alignment::Left)
                .borders(Borders::default().modifiers(BorderType::Rounded))
                .cursor_line_style(Style::default())
                .cursor_style(Style::default().add_modifier(TextModifiers::REVERSED))
                .line_number_style(Style::default().fg(Color::Gray).bg(Color::DarkGray))
                .max_histories(64)
                .scroll_step(4)
                .inactive(Style::default().fg(Color::DarkGray)),
        }
    }

    /// Retrieves the content from the SQL input component.
    pub fn get_content(&mut self) -> Vec<String> {
        match self.perform(Cmd::Submit) {
            CmdResult::Submit(data) => data
                .unwrap_vec()
                .iter()
                .map(|v| v.clone().unwrap_string())
                .collect(),
            _ => Vec::new(),
        }
    }
}

impl MockComponent for ComponentSQLInput<'_> {
    /// Renders the SQL input on the screen.
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.component.view(frame, area)
    }

    /// Queries an attribute of the input component.
    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    /// Sets an attribute for the input component.
    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value.clone());
    }

    /// Retrieves the state of the input component.
    fn state(&self) -> State {
        self.component.state()
    }

    /// Executes commands on the input component.
    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<TuiMsg, TuiUserEvent> for ComponentSQLInput<'_> {
    /// Handles keyboard events for SQL input.
    fn on(&mut self, ev: Event<TuiUserEvent>) -> Option<TuiMsg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers,
            }) => match modifiers {
                KeyModifiers::CONTROL => self.perform(Cmd::GoTo(Position::Begin)),
                KeyModifiers::ALT => self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_PARAGRAPH_BACK)),
                KeyModifiers::SHIFT => self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_BACK)),
                KeyModifiers::NONE => self.perform(Cmd::Move(Direction::Left)),
                _ => CmdResult::None,
            },
            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::ALT,
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers,
            }) => match modifiers {
                KeyModifiers::CONTROL => self.perform(Cmd::GoTo(Position::End)),
                KeyModifiers::ALT => self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_PARAGRAPH_FORWARD)),
                KeyModifiers::SHIFT => self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_FORWARD)),
                KeyModifiers::NONE => self.perform(Cmd::Move(Direction::Right)),
                _ => CmdResult::None,
            },
            Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::GoTo(Position::End)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('f'),
                modifiers: KeyModifiers::ALT,
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers,
            }) => match modifiers {
                KeyModifiers::CONTROL => return Some(TuiMsg::SQLInputHistoryBack),
                KeyModifiers::NONE => self.perform(Cmd::Move(Direction::Up)),
                _ => CmdResult::None,
            },
            Event::Keyboard(KeyEvent {
                code: Key::Char('p'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                return Some(TuiMsg::SQLInputHistoryBack);
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers,
            }) => match modifiers {
                KeyModifiers::CONTROL => return Some(TuiMsg::SQLInputHistoryForward),
                KeyModifiers::NONE => self.perform(Cmd::Move(Direction::Down)),
                _ => CmdResult::None,
            },
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                return Some(TuiMsg::SQLInputHistoryForward);
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_TOP)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_BOTTOM))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('d'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Cancel), // Delete next char
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Delete), // Delete previous char
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_REDO)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_UNDO)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('v'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_PASTE)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('x'),
                modifiers: KeyModifiers::CONTROL,
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                let query = self.get_content().join("\n");
                return Some(TuiMsg::DatabaseRequestByQuery(query));
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Type(ch)),
            _ => CmdResult::None,
        };
        Some(TuiMsg::Redraw)
    }
}
