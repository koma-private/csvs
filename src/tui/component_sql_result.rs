use crate::db::statement_result::StatementPagedResult;
use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;
use std::time::Duration;

use tui_realm_stdlib::Table;
use tuirealm::command::{Cmd, Direction};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{
    Alignment, BorderType, Borders, Color, PropPayload, PropValue, Style, TableBuilder, TextSpan,
};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent};

/// TUI component for displaying SQL query results.
#[derive(MockComponent)]
pub struct ComponentSQLResult {
    /// The table component used to render the SQL results.
    pub component: Table, // Table rendering the query results
    pub current_page_index: usize,    // Current page number
    pub elapsed: Duration,            // Time taken for query execution
    pub page_size: usize,             // Rows per page
    pub page_upper_limit: usize,      // Max page number
    pub total_rows: usize,            // Total rows in the result set
    pub total_columns: Option<usize>, // Total number of columns
}

impl ComponentSQLResult {
    /// Initializes the component with default settings.
    pub fn new() -> Self {
        let component = Table::default()
            .title("SQL Result (F3)", Alignment::Left)
            .borders(Borders::default().modifiers(BorderType::Rounded))
            .scroll(true)
            .highlighted_color(Color::Yellow)
            .inactive(Style::default().fg(Color::DarkGray));

        Self {
            component,
            current_page_index: 0,
            elapsed: Duration::default(),
            page_size: 0,
            page_upper_limit: 0,
            total_columns: None,
            total_rows: 0,
        }
    }

    /// Updates and returns result information.
    pub fn update_sql_result_info(&self) -> String {
        let current_row = self
            .current_page_index
            .saturating_mul(self.page_size)
            .saturating_add(self.component.states.list_index)
            .saturating_add(1);
        let total_columns = self
            .total_columns
            .map(|cols| cols.to_string())
            .unwrap_or_else(|| "N/A".to_string());

       format!(
            "Row:{} Page:{} Page Size:{} Columns:{} Elapsed:{}msecs",
            padded_number(current_row, self.total_rows),
            padded_number(
                self.current_page_index.saturating_add(1),
                self.page_upper_limit.saturating_add(1)
            ),
            self.page_size,
            total_columns,
            self.elapsed.as_millis()
        )
    }

    /// Loads new content into the component.
    pub fn load_content(&mut self, result: StatementPagedResult) {
        self.current_page_index = result.current_page_index;
        self.elapsed = result.elapsed;
        self.page_size = result.page_size;
        self.page_upper_limit = result.page_upper_limit;
        self.total_columns = result.total_columns;
        self.total_rows = result.total_rows;

        let header = result
            .header
            .iter()
            .map(|v| PropValue::Str(v.clone()))
            .collect();
        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(header)),
        );

        let mut tb = TableBuilder::default();
        let mut is_first_row = true;
        for row_index in 0..result.rows.len() {
            if is_first_row {
                is_first_row = false;
            } else {
                tb.add_row();
            }
            for column in result.rows[row_index].clone() {
                tb.add_col(TextSpan::from(column));
            }
        }

        self.attr(Attribute::Content, AttrValue::Table(tb.build()));

        self.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::One(PropValue::Usize(
                result.initial_row_position,
            ))),
        ); // Reset position of selected row
    }

    /// Determines if scrolling requires a page refresh.
    pub fn will_scroll_invoke_page_refresh(&self, direction: Direction) -> Option<usize> {
        let scroll_step = self
            .query(Attribute::ScrollStep)
            .unwrap_or(AttrValue::Length(8));
        let scroll_step = scroll_step.unwrap_length();

        match direction {
            Direction::Down => {
                let diff: isize = (self.component.states.list_index.saturating_add(scroll_step)
                    as isize)
                    .saturating_sub(self.page_size as isize);

                if diff > 0 {
                    if self.current_page_index <= self.page_upper_limit {
                        Some(diff as usize) // Initial position is `diff`
                    } else {
                        Some(self.component.states.list_len.saturating_sub(1)) // Initial position is bottom
                    }
                } else {
                    None // No page refreshing
                }
            }
            Direction::Up => {
                let diff = (self.component.states.list_index as isize)
                    .saturating_sub(scroll_step as isize);

                if diff < 0 {
                    if self.current_page_index == 0 {
                        None
                    } else {
                        Some((self.page_size as isize).saturating_add(diff) as usize)
                    }
                } else {
                    None // No page refreshing
                }
            }
            _ => None,
        }
    }

    pub fn will_move_invoke_page_refresh(&self, direction: Direction) -> bool {
        match direction {
            Direction::Down => {
                (self.current_page_index <= self.page_upper_limit)
                    && (self.component.states.list_index
                        == self.component.states.list_len.saturating_sub(1))
            }
            Direction::Up => {
                (self.current_page_index > 0) && (self.component.states.list_index == 0)
            }
            _ => false,
        }
    }
}

impl Component<TuiMsg, TuiUserEvent> for ComponentSQLResult {
    /// Handles user input events.
    fn on(&mut self, ev: Event<TuiUserEvent>) -> Option<TuiMsg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                if self.will_move_invoke_page_refresh(Direction::Down) {
                    Some(TuiMsg::DatabaseRequestByPage(
                        0,
                        self.current_page_index.saturating_add(1),
                    ))
                } else {
                    self.perform(Cmd::Move(Direction::Down));
                    let info = self.update_sql_result_info();
                    Some(TuiMsg::UpdateSQLResultInfo(info))
                }
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('p'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                if self.will_move_invoke_page_refresh(Direction::Up) {
                    Some(TuiMsg::DatabaseRequestByPage(
                        self.page_size.saturating_sub(1),
                        self.current_page_index.saturating_sub(1),
                    ))
                } else {
                    self.perform(Cmd::Move(Direction::Up));
                    let info = self.update_sql_result_info();
                    Some(TuiMsg::UpdateSQLResultInfo(info))
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => {
                if let Some(initial_position) =
                    self.will_scroll_invoke_page_refresh(Direction::Down)
                {
                    Some(TuiMsg::DatabaseRequestByPage(
                        initial_position,
                        self.current_page_index.saturating_add(1),
                    ))
                } else {
                    self.perform(Cmd::Scroll(Direction::Down));
                    let info = self.update_sql_result_info();
                    Some(TuiMsg::UpdateSQLResultInfo(info))
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => {
                if let Some(initial_position) = self.will_scroll_invoke_page_refresh(Direction::Up)
                {
                    Some(TuiMsg::DatabaseRequestByPage(
                        initial_position,
                        self.current_page_index.saturating_sub(1),
                    ))
                } else {
                    self.perform(Cmd::Scroll(Direction::Up));
                    let info = self.update_sql_result_info();
                    Some(TuiMsg::UpdateSQLResultInfo(info))
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => Some(TuiMsg::DatabaseRequestByPage(0, 0)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                Some(TuiMsg::DatabaseRequestByPage(
                    self.page_upper_limit.saturating_add(1),
                    self.page_upper_limit,
                ))
            }
            Event::User(TuiUserEvent::ResponseDatabasePagedResult(result)) => {
                self.load_content(result);
                let info = self.update_sql_result_info();
                Some(TuiMsg::UpdateSQLResultInfo(info))
            }
            _ => None,
        }
    }
}

/// Formats numbers with leading zeros based on max digits.
fn padded_number(current: usize, max: usize) -> String {
    let max_length = max.checked_ilog10().unwrap_or(0).saturating_add(1) as usize;

    let padded = format!("{:01$}", current, max_length);
    format!("{}/{}", padded, max)
}

#[test]
fn test_padded_number() {
    println!("{}", padded_number(4, 1000));
}
