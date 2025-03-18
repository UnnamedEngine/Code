use strum::Display;

/// Events produced by a window.
#[derive(Debug, Display)]
pub enum WindowEvent {
    /// The window was resized.
    Resized(u32, u32),
}
