use crate::args::Args;
use crate::db::statement_result::StatementResult;
use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;

use tui_realm_stdlib::Phantom;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::ratatui::layout::Rect;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, State};

/// Stores the most recent query result.
pub(crate) static QUERY_RESULT: std::sync::LazyLock<
    std::sync::Arc<std::sync::RwLock<StatementResult>>,
> = std::sync::LazyLock::new(|| {
    std::sync::Arc::new(std::sync::RwLock::new(StatementResult::default()))
});

/// Tracks if a query is currently executing.
pub(crate) static IS_QUERYING: std::sync::LazyLock<std::sync::Arc<std::sync::RwLock<bool>>> =
    std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::RwLock::new(false)));

/// Tracks if a result is currently being saved.
pub(crate) static IS_SAVING: std::sync::LazyLock<std::sync::Arc<std::sync::RwLock<bool>>> =
    std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::RwLock::new(false)));

/// Handles database events and TUI interactions.
pub struct DatabaseEventListener {
    component: Phantom,

    /// Command-line arguments.
    pub args: Args,
    /// SQLite's connection pool.
    pub pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    /// Sender for user events.
    pub sender_user_event: std::sync::mpsc::Sender<Event<TuiUserEvent>>,
}

impl MockComponent for DatabaseEventListener {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.component.view(frame, area)
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value)
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl DatabaseEventListener {
    /// Creates a new database event listener.
    pub fn new(
        pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
        args: Args,
        sender_user_event: std::sync::mpsc::Sender<Event<TuiUserEvent>>,
    ) -> Self {
        Self {
            component: Default::default(),
            args,
            pool,
            sender_user_event,
        }
    }
}

impl Component<TuiMsg, TuiUserEvent> for DatabaseEventListener {
    /// Handles TUI events related to database operations.
    ///
    /// # Arguments
    /// * `ev` - The event to handle (e.g., user actions or UI events).
    ///
    /// # Returns
    /// A `TuiMsg` or `None` depending on the event handled.
    fn on(&mut self, ev: Event<TuiUserEvent>) -> Option<TuiMsg> {
        match ev {
            Event::User(TuiUserEvent::RequestDatabaseByQuery(query)) => self.sql_execute(query),
            Event::User(TuiUserEvent::RequestDatabaseByPage(
                initial_row_position,
                current_page_index,
            )) => {
                let query_result = QUERY_RESULT.read().unwrap();

                if let Some(paged_result) = Self::sql_result_by_page(
                    &query_result,
                    initial_row_position,
                    current_page_index,
                ) {
                    self.sender_user_event
                        .send(Event::User(TuiUserEvent::ResponseDatabasePagedResult(
                            paged_result,
                        )))
                        .expect("Failed to send paged result event");
                }
                None
            }
            Event::User(TuiUserEvent::RequestDatabaseAvailableTables) => self.available_tables(),
            Event::User(TuiUserEvent::RequestDatabaseSaveResult(filename)) => {
                self.sql_result_save_file(filename)
            }
            _ => {
                None // Ignore unhandled events
            }
        }
    }
}
