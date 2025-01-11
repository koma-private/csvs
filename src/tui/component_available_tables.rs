use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use tui_realm_stdlib::Table;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, Style, TableBuilder, TextSpan};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent};

#[derive(MockComponent)]
/// TUI component for displaying available database tables.
pub struct ComponentAvailableTables {
    /// Table component to render tables.
    pub component: Table,
}

impl Default for ComponentAvailableTables {
    /// Creates a default instance with predefined styles.
    fn default() -> Self {
        // Configure the table component with default styling and behavior.
        let component = Table::default()
            .title("Available Tables (F1)", Alignment::Left)
            .borders(Borders::default().modifiers(BorderType::Rounded))
            .scroll(true)
            .rewind(true)
            .highlighted_color(Color::Yellow)
            .inactive(Style::default().fg(Color::DarkGray));

        Self { component }
    }
}

impl Component<TuiMsg, TuiUserEvent> for ComponentAvailableTables {
    /// Handles events for navigation and selection.
    fn on(&mut self, ev: Event<TuiUserEvent>) -> Option<TuiMsg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if let Some(AttrValue::Table(table)) = self.query(Attribute::Content) {
                    if let Some(row) = table.get(self.component.states.list_index) {
                        if let Some(selected) = row.first() {
                            return Some(TuiMsg::AvailableTablesSelected(selected.content.clone()));
                        }
                    }
                }
                CmdResult::None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('i'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                if let Some(AttrValue::Table(table)) = self.query(Attribute::Content) {
                    if let Some(row) = table.get(self.component.states.list_index) {
                        if let Some(selected) = row.first() {
                            return Some(TuiMsg::DatabaseRequestTableInfo(selected.content.clone()));
                        }
                    }
                }
                CmdResult::None
            }
            Event::User(TuiUserEvent::ResponseDatabaseAvailableTables(available_tables)) => {
                let mut tb = TableBuilder::default();
                for (tables_index, available_table) in available_tables.iter().enumerate() {
                    if tables_index > 0 {
                        tb.add_row();
                    }
                    tb.add_col(TextSpan::from(available_table.clone()));
                }

                self.attr(Attribute::Content, AttrValue::Table(tb.build()));

                return Some(TuiMsg::Redraw);
            }
            _ => {
                CmdResult::None // Ignore unhandled events
            }
        };
        Some(TuiMsg::Redraw) // Return None message for default case
    }
}
