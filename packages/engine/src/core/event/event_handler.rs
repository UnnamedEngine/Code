use super::Event;

pub type RawCallback<T> = Box<dyn FnMut(&mut T, &Event)>;

/// Helper that facilitates the creation of event handler callbacks that can
/// interact with the passed data at a later stage.
pub struct EventHandler<T> {
    callback: Option<RawCallback<T>>,
}

impl<T> EventHandler<T> {
    pub fn new() -> Self {
        Self { callback: None }
    }

    /// Sets the callback for this instance.
    pub fn set_callback<F>(&mut self, handler: F)
    where
        F: FnMut(&mut T, &Event) + 'static,
    {
        self.callback = Some(Box::new(handler));
    }

    /// Does a iteration for this instance.
    pub fn step(&mut self, data: &mut T, event: &Event) {
        if let Some(callback) = self.callback.as_mut() {
            callback(data, event);
        }
    }
}

impl<T> Default for EventHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}
