use strum::Display;

/// Events produced by the engine.
#[derive(Debug, Display)]
pub enum EngineEvent {
    /// Just started.
    Started,
    /// Preparing for shutdown.
    Shutdown,
    /// Stopped.
    Stopped,
}
