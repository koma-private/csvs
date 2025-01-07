use crate::tui::tui_user_event::TuiUserEvent;
use std::sync::mpsc::TryRecvError;
use tuirealm::listener::{ListenerResult, Poll};
use tuirealm::Event;

/// UserEventPort handles user events and forwards them to the TUI application.
#[derive(Debug)]
pub struct UserEventPort {
    receiver: std::sync::mpsc::Receiver<Event<TuiUserEvent>>,
}

impl UserEventPort {
    /// Constructs a new UserEventPort instance.
    ///
    /// # Arguments
    /// * `receiver` - Receiver end of the mpsc channel to handle incoming user events.
    pub fn new(receiver: std::sync::mpsc::Receiver<Event<TuiUserEvent>>) -> Self {
        Self { receiver }
    }
}

impl Poll<TuiUserEvent> for UserEventPort {
    /// Polls for user events from the receiver channel.
    ///
    /// # Returns
    /// * `Ok(Some(Event))` if a user event is available.
    /// * `Ok(None)` if no events are present.
    /// * `Err(ListenerError::PollFailed)` if the receiver is disconnected.
    fn poll(&mut self) -> ListenerResult<Option<Event<TuiUserEvent>>> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(err) => match err {
                TryRecvError::Empty => Ok(None), // No events to process.
                TryRecvError::Disconnected => Err(tuirealm::ListenerError::PollFailed), // Channel disconnected.
            },
        }
    }
}
