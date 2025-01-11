use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use crate::db::table_info::TableInfo;
use tui_realm_stdlib::Table;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, Style, TableBuilder, TextSpan};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Clear;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, State};

/// TUI component for displaying SQL query results.
pub struct ComponentTableInfoDialog {
    /// The table component used to render the SQL results.
    pub component: Table, // Table rendering the query results
}

impl ComponentTableInfoDialog {
    /// Initializes the component with default settings.
    pub fn new(table_name: &str, table_infos: Vec<TableInfo>) -> Self {
        let table = Self::load_content(table_infos);

        let component = Table::default()
            .title(table_name, Alignment::Left)
            .table(table)
            .borders(Borders::default().modifiers(BorderType::Rounded))
            .scroll(true)
            .highlighted_color(Color::Yellow)
            .inactive(Style::default().fg(Color::DarkGray));

        Self { component }
    }

    /// Loads new content into the component.
    pub fn load_content(table_infos: Vec<TableInfo>) -> Vec<Vec<TextSpan>> {
        let mut tb = TableBuilder::default();
        let mut is_first_row = true;
        for table_info in table_infos {
            if is_first_row {
                is_first_row = false;
            } else {
                tb.add_row();
            }
            tb.add_col(TextSpan::from(table_info.name));
            tb.add_col(TextSpan::from(table_info.data_type));
            tb.add_col(TextSpan::from(if table_info.notnull {
                "NOT NULL"
            } else {
                ""
            }));
            if let Some(dflt_value) = table_info.dflt_value {
                tb.add_col(TextSpan::from(dflt_value));
            } else {
                tb.add_col(TextSpan::new(""));
            }
            tb.add_col(TextSpan::from(if table_info.pk { "PK" } else { "" }));
        }
        tb.build()
    }
}

impl MockComponent for ComponentTableInfoDialog {
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

impl Component<TuiMsg, TuiUserEvent> for ComponentTableInfoDialog {
    /// Handles user input events.
    fn on(&mut self, ev: Event<TuiUserEvent>) -> Option<TuiMsg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if let Some(AttrValue::Table(table)) = self.query(Attribute::Content) {
                    if let Some(row) = table.get(self.component.states.list_index) {
                        if let Some(selected) = row.first() {
                            return Some(TuiMsg::TableInfoColumnSelected(selected.content.clone()));
                        }
                    }
                }
                CmdResult::None
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. })
            | Event::Keyboard(KeyEvent { code: Key::Tab, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => return Some(TuiMsg::TableInfoDialogClose),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('p'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Move(Direction::Up)),
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
            _ => CmdResult::None,
        };
        Some(TuiMsg::Redraw)
    }
}
