use std::sync::mpsc::{channel, Receiver, Sender};

use strum::Display;

pub mod engine_event;
pub mod event_handler;
pub mod keyboard_event;
pub mod mouse_event;
pub mod window_event;

pub use engine_event::EngineEvent;
pub use event_handler::EventHandler;
pub use keyboard_event::KeyboardEvent;
pub use mouse_event::MouseEvent;
pub use window_event::WindowEvent;

/// Main enum that defines all our events.
///
/// There **is** a naming convention for any `Event`:
///
/// - past-sentence names are refered to events that already occurred.
///
/// Other events are yet to occurr and reacting to them can have some sort of
/// influence on the final result.
///
/// **Events must contain only simple data.**
#[derive(Debug, Display)]
pub enum Event {
    /// Events produced by the engine.
    Engine(EngineEvent),
    /// Events produced by a window.
    Window(WindowEvent),
    /// Events produced by the keyboard.
    Keyboard(KeyboardEvent),
    /// Events produced by the mouse.
    Mouse(MouseEvent),

    /// Only used during tests.
    #[cfg(test)]
    Dummy,
}

/// Creates new dispatcher and consumer that are linked together.
///
/// Only one consumer can exist, while multiple instances of a dispatcher can be
/// used by calling `.clone()` on it.
pub fn create_handler() -> (Dispatcher, Consumer) {
    let (sender, receiver) = channel();

    let dispatcher = Dispatcher::new(sender);
    let consumer = Consumer::new(receiver);

    (dispatcher, consumer)
}

/// Helper that dispatches events.
#[derive(Debug, Clone)]
pub struct Dispatcher {
    sender: Sender<Event>,
}

impl Dispatcher {
    fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }

    /// Sends the event to be processed by the consumer.
    pub fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}

/// Helper that consumes events.
pub struct Consumer {
    receiver: Receiver<Event>,
}

impl Consumer {
    fn new(receiver: Receiver<Event>) -> Self {
        Self { receiver }
    }

    /// Tries to receive an event. If no events are found it returns `None`.
    pub fn poll(&self) -> Option<Event> {
        match self.receiver.try_recv() {
            Ok(event) => Some(event),
            Err(err) => match err {
                std::sync::mpsc::TryRecvError::Empty => None,
                _ => {
                    log::error!("Failed to receive event: {err}");
                    None
                }
            },
        }
    }
}
