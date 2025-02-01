use engine::EngineState;
use thiserror::Error;

pub mod application;
pub mod engine;
pub mod event;
pub mod scheduler;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid State: expected {0} got {1}")]
    InvalidState(EngineState, EngineState),
    #[error("Event Loop Error: {0}")]
    EventLoopError(#[from] winit::error::EventLoopError),
}
