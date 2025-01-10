use crate::args_util::args::Args;
use crate::db::statement_result::StatementPagedResult;
use crate::tui::component_available_tables::ComponentAvailableTables;
use crate::tui::component_message_dialog::ComponentMessageDialogType;
use crate::tui::component_shortcut_menu::ComponentShortcutMenu;
use crate::tui::component_sql_input::ComponentSQLInput;
use crate::tui::component_sql_result::ComponentSQLResult;
use crate::tui::component_sql_result_info::ComponentSQLResultInfo;
use crate::tui::database_event_listener::DatabaseEventListener;
use crate::tui::event_listener::EventListener;
use crate::tui::input_history::InputHistory;
use crate::tui::tui_id::TuiId;
use crate::tui::tui_msg::TuiMsg;
use crate::tui::tui_user_event::TuiUserEvent;
use crate::tui::user_event_port::UserEventPort;
use std::io::stdout;
use std::time::Duration;

use tracing::debug;
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::ratatui::crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use tuirealm::ratatui::crossterm::execute;
use tuirealm::ratatui::crossterm::terminal::supports_keyboard_enhancement;
use tuirealm::ratatui::layout::{Constraint, Flex, Layout, Rect};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge};
use tuirealm::{Application, Event, EventListenerCfg, Sub, SubClause, SubEventClause, Update};

/// Model for managing the TUI application's state and behavior.
pub struct TuiModel<T>
where
    T: TerminalAdapter,
{
    pub app: Application<TuiId, TuiMsg, TuiUserEvent>, // TUI application instance.
    pub input_history: InputHistory,                   // Tracks user input history.
    pub keyboard_enhancement_enabled: bool, // Indicates if keyboard enhancement is active.
    pub menu_stack: Vec<TuiId>,             // Stack for navigating the TUI menu.
    pub sender_user_event: std::sync::mpsc::Sender<Event<TuiUserEvent>>, // User event sender.
    pub should_quit: bool,                  // Flag to terminate the application loop.
    pub should_redraw: bool,                // Flag to trigger UI redraw.
    pub terminal: TerminalBridge<T>,        // Handles terminal interactions.
}

impl TuiModel<CrosstermTerminalAdapter> {
    /// Initializes the TUI model with a database pool and application arguments.
    pub fn new(
        pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
        args: Args,
    ) -> anyhow::Result<Self> {
        let mut terminal = TerminalBridge::init_crossterm()?;
        terminal.enter_alternate_screen()?;
        terminal.enable_raw_mode()?;

        let keyboard_enhancement_enabled = if cfg!(unix) {
            if matches!(supports_keyboard_enhancement(), Ok(true)) {
                execute!(
                    stdout(),
                    PushKeyboardEnhancementFlags(
                        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                            | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                    )
                )?;
                true
            } else {
                false
            }
        } else {
            cfg!(windows) // Windows can receive SHIFT-Enter key event
        };

        let (app, sender_user_event) = Self::init_app(pool, args)?;

        Ok(Self {
            app,
            input_history: Default::default(),
            keyboard_enhancement_enabled,
            menu_stack: vec![],
            sender_user_event,
            should_quit: false,
            should_redraw: true,
            terminal,
        })
    }

    /// Restores the terminal to its original state.
    pub(crate) fn restore_terminal(mut self) -> anyhow::Result<()> {
        if cfg!(unix) && self.keyboard_enhancement_enabled {
            execute!(stdout(), PopKeyboardEnhancementFlags)?;
        }
        self.terminal.disable_raw_mode()?;
        self.terminal.leave_alternate_screen()?;
        if cfg!(windows) {
            self.terminal.clear_screen()?;
        }
        Ok(())
    }
}

type InitAppType = (
    Application<TuiId, TuiMsg, TuiUserEvent>,
    std::sync::mpsc::Sender<Event<TuiUserEvent>>,
);

impl<T> TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Renders the TUI application.
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .draw(|f| {
                let rects_vertical = Layout::vertical([
                    Constraint::Length(8),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .split(f.area());

                let rects_horizontal =
                    Layout::horizontal([Constraint::Length(30), Constraint::Percentage(100)])
                        .split(rects_vertical[0]);

                self.app
                    .view(&TuiId::AvailableTables, f, rects_horizontal[0]);
                self.app.view(&TuiId::SQLInput, f, rects_horizontal[1]);
                self.app.view(&TuiId::SQLResult, f, rects_vertical[1]);
                self.app.view(&TuiId::SQLResultInfo, f, rects_vertical[2]);
                self.app.view(&TuiId::ShortcutMenu, f, rects_vertical[3]);
                self.app
                    .view(&TuiId::MessageDialog, f, popup_area(f.area(), 60, 8));
                self.app
                    .view(&TuiId::ProgressDialog, f, popup_area(f.area(), 60, 8));
                self.app.view(
                    &TuiId::SQLResultFilenameInputDialog,
                    f,
                    popup_area(f.area(), 60, 3),
                );
            })
            .is_ok());
    }

    /// Initializes the TUI application components.
    fn init_app(
        pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
        args: Args,
    ) -> anyhow::Result<InitAppType> {
        let (sender_user_event, receiver_user_event) =
            std::sync::mpsc::channel::<Event<TuiUserEvent>>();

        debug!("Initializing TUI application.");
        let mut app: Application<TuiId, TuiMsg, TuiUserEvent> = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .add_port(
                    Box::new(UserEventPort::new(receiver_user_event)),
                    Duration::from_millis(50),
                    3,
                )
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        app.mount(
            TuiId::ShortcutMenu,
            Box::new(ComponentShortcutMenu::default()),
            Vec::new(),
        )?;

        app.mount(
            TuiId::SQLInput,
            Box::new(ComponentSQLInput::new("")),
            Vec::new(),
        )?;

        app.mount(
            TuiId::AvailableTables,
            Box::new(ComponentAvailableTables::default()),
            Vec::new(),
        )?;

        app.mount(
            TuiId::SQLResult,
            Box::new(ComponentSQLResult::new()),
            vec![Sub::new(
                SubEventClause::User(TuiUserEvent::ResponseDatabasePagedResult(
                    StatementPagedResult::default(),
                )),
                SubClause::Always,
            )],
        )?;

        app.mount(
            TuiId::SQLResultInfo,
            Box::new(ComponentSQLResultInfo::default()),
            Vec::new(),
        )?;

        app.mount(
            TuiId::KeyEventListener,
            Box::new(EventListener::default()),
            vec![
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                    }),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Char('s'),
                        modifiers: KeyModifiers::CONTROL,
                    }),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Function(1),
                        modifiers: KeyModifiers::NONE,
                    }),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Function(2),
                        modifiers: KeyModifiers::NONE,
                    }),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Function(3),
                        modifiers: KeyModifiers::NONE,
                    }),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Tab,
                        modifiers: KeyModifiers::NONE,
                    }),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::BackTab,
                        modifiers: KeyModifiers::SHIFT,
                    }),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::User(TuiUserEvent::MessageDialogShow(
                        TuiId::MessageDialog,
                        ComponentMessageDialogType::Info,
                        String::new(),
                    )),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::User(TuiUserEvent::ProgressDialogClose),
                    SubClause::Always,
                ),
            ],
        )?;

        app.mount(
            TuiId::DatabaseEventListener,
            Box::new(DatabaseEventListener::new(
                pool,
                args,
                sender_user_event.clone(),
            )),
            vec![
                Sub::new(
                    SubEventClause::User(TuiUserEvent::RequestDatabaseAvailableTables),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::User(TuiUserEvent::RequestDatabaseByQuery(String::new())),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::User(TuiUserEvent::RequestDatabaseSaveResult(String::new())),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::User(TuiUserEvent::RequestDatabaseByPage(0, 0)),
                    SubClause::Always,
                ),
            ],
        )?;

        app.active(&TuiId::AvailableTables)?;
        Ok((app, sender_user_event))
    }
}

impl<T> Update<TuiMsg> for TuiModel<T>
where
    T: TerminalAdapter,
{
    /// Handles updates based on received messages.
    /// Updates application state, triggers redraws, and handles user interactions.
    ///
    /// # Arguments
    /// * `msg` - Optional message to process.
    ///
    /// # Returns
    /// * An optional `TuiMsg` for further processing.
    fn update(&mut self, msg: Option<TuiMsg>) -> Option<TuiMsg> {
        if let Some(msg) = msg {
            self.should_redraw = true; // Flag to redraw the UI.
            match msg {
                TuiMsg::AppClose => {
                    if !self.is_dialog_displayed() {
                        self.should_quit = true;
                    }
                    None
                }
                TuiMsg::AvailableTablesSelected(selected) => {
                    self.available_tables_selected(selected)
                }
                TuiMsg::MessageDialogClose => self.message_dialog_close(),
                TuiMsg::MessageDialogShow(tui_id, dialog_type, message) => {
                    self.message_dialog_show(tui_id, dialog_type, message)
                }
                TuiMsg::MoveInputFocus(tui_id) => self.move_input_focus(tui_id),
                TuiMsg::ForwardInputFocus => self.forward_input_focus(),
                TuiMsg::BackInputFocus => self.back_input_focus(),
                TuiMsg::ShortcutMenuUpdate(tui_id) => self.shortcut_menu_update(tui_id),
                TuiMsg::DatabaseRequestAvailableTables => {
                    self.sender_user_event
                        .send(Event::User(TuiUserEvent::RequestDatabaseAvailableTables))
                        .expect("Failed to send request for available tables");
                    None
                }
                TuiMsg::DatabaseRequestByQuery(query) => {
                    self.sql_input_history_save(&query);
                    self.sender_user_event
                        .send(Event::User(TuiUserEvent::RequestDatabaseByQuery(query)))
                        .expect("Failed to send query request");
                    None
                }
                TuiMsg::DatabaseRequestByPage(initial_row_position, current_page_index) => {
                    self.sender_user_event
                        .send(Event::User(TuiUserEvent::RequestDatabaseByPage(
                            initial_row_position,
                            current_page_index,
                        )))
                        .expect("Failed to send request for paginated results");
                    None
                }
                TuiMsg::DatabaseRequestSaveResult(filename) => {
                    // Unmount the filename input dialog
                    self.app
                        .umount(&TuiId::SQLResultFilenameInputDialog)
                        .expect("Failed to unmount SQL result filename input dialog");

                    self.sender_user_event
                        .send(Event::User(TuiUserEvent::RequestDatabaseSaveResult(
                            filename,
                        )))
                        .expect("Failed to send save result request");
                    None
                }
                TuiMsg::SQLInputValue(value) => self.sql_input_value(value),
                TuiMsg::SQLInputHistoryForward => self.sql_input_history_forward(),
                TuiMsg::SQLInputHistoryBack => self.sql_input_history_back(),
                TuiMsg::ProgressDialogShow(message) => self.progress_dialog_show(message),
                TuiMsg::ProgressDialogClose => self.progress_dialog_close(),
                TuiMsg::SQLResultFilenameInputDialogClose => {
                    self.sql_result_filename_input_dialog_close()
                }
                TuiMsg::SQLResultFilenameInputDialogShow => {
                    self.sql_result_filename_input_dialog_show()
                }
                TuiMsg::UIElementDarken(flag) => self.ui_elements_darken(flag),
                TuiMsg::UpdateSQLResultInfo(info) => self.update_sql_result_info(info),
                TuiMsg::Redraw => None,
            }
        } else {
            None
        }
    }
}

/// Calculates the popup area for dialogs based on the provided dimensions.
fn popup_area(area: Rect, length_x: u16, length_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(length_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(length_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
