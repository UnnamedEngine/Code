use std::collections::VecDeque;

use strum::Display;

use super::{
    event::{self, Event},
    scheduler::{pool::WorkerPool, worker::WorkerInstruction},
    Error,
};

/// All the possible states a `Engine` can be at.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    /// Currently stopped and can only be start with `::start()`.
    Stopped,
    /// Currently starting and will change to `EngineState::Running` once the
    /// starting process ends.
    Starting,
    /// Currently running and can only change to `EngineState::Stopping`.
    Running,
    /// Currently stopping and will change to `EngineState::Stopped` once the
    /// stopping process ends.
    Stopping,
}

pub struct EngineData {
    /// Flags the current engine state.
    pub state: EngineState,
    /// Event dispatcher that can be freely cloned anywhere.
    pub event_dispatcher: event::Dispatcher,
}

pub struct Engine {
    /// Shared data.
    data: EngineData,
    /// Event consumer that will continuosly poll for events.
    event_consumer: event::Consumer,
    /// Worker pool.
    worker_pool: WorkerPool,
    /// Contains the events that are already handled by the engine's internals
    /// and can be used by other areas.
    ready_events: VecDeque<Event>,
}

impl Default for Engine {
    fn default() -> Self {
        // The logger is started here to make sure we have logging always
        // available

        // We do not want a env_logger during tests
        #[cfg(not(test))]
        {
            // Read the env values that configure the logger
            let env = env_logger::Env::default()
                .filter_or("MY_LOG_LEVEL", "info")
                .write_style_or("MY_LOG_STYLE", "always");

            // Initialize the logger from env
            env_logger::init_from_env(env);
        }

        let (event_dispatcher, event_consumer) = event::create_handler();

        let data = EngineData {
            state: EngineState::Stopped,
            event_dispatcher,
        };

        let worker_pool = WorkerPool::default();

        let ready_events = VecDeque::default();

        Self {
            data,
            event_consumer,
            worker_pool,
            ready_events,
        }
    }
}

impl Engine {
    /// Start.
    pub fn run(&mut self) -> Result<(), Error> {
        match self.data.state {
            EngineState::Stopped => {
                self.data.state = EngineState::Starting;
                self.start();
            }
            _ => {
                return Err(Error::InvalidState(
                    EngineState::Stopped,
                    self.data.state,
                ))
            }
        }

        Ok(())
    }

    /// Stop.
    pub fn shutdown(&mut self) -> Result<(), Error> {
        match self.data.state {
            EngineState::Running => {
                self.data.state = EngineState::Stopping;
                self.stop();
            }
            _ => {
                return Err(Error::InvalidState(
                    EngineState::Running,
                    self.data.state,
                ))
            }
        }

        Ok(())
    }

    /// Runs one iteration.
    pub fn step(&mut self) {
        self.update();
    }

    /// Internal function that handles the engine's start.
    pub fn start(&mut self) {
        // TODO: there should be something here to start the engine
        self.data.state = EngineState::Running;
        log::info!("Successfully started engine");
    }

    /// Internal function that handles the engine's stop.
    pub fn stop(&mut self) {
        // TODO: there should be something hete to stop the engine
        self.data.state = EngineState::Stopped;
        log::info!("Successfully stopped engine");
    }

    /// Internal function that updates the engine.
    fn update(&mut self) {
        self.handle_all_events();
    }

    /// Gets the current state.
    pub fn state(&self) -> EngineState {
        self.data.state
    }

    /// Dispatches the passed event.
    pub fn dispatch(&self, event: Event) {
        self.data.event_dispatcher.send(event);
    }

    /// Sends an instruction to an available worker.
    pub fn instruct(&mut self, instruction: WorkerInstruction) {
        match self.worker_pool.send(instruction) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to send instruction to worker: {err}");
            }
        }
    }

    /// Handle all pending events.
    fn handle_all_events(&mut self) {
        let mut pending = true;
        while pending {
            pending = self.handle_event();
        }
    }

    /// Handle a single event.
    pub fn handle_event(&mut self) -> bool {
        if let Some(event) = self.event_consumer.poll() {
            match &event {
                event::Event::Engine(engine_event) => match engine_event {
                    event::engine_event::EngineEvent::Started => {
                        log::info!("Successfully started engine!");
                    }
                    event::engine_event::EngineEvent::Shutdown => {
                        log::info!("Engine preparing for graceful shutdown!");
                    }
                    event::engine_event::EngineEvent::Stopped => {
                        log::info!("Engine gracefully stopped!");
                        log::info!("See you again :D");
                    }
                },

                // Ignore mouse events just so we don't spam the log that much
                event::Event::Mouse(_) => {}

                #[cfg(test)]
                event::Event::Dummy => {}

                _ => {
                    log::warn!(
                        "Internal event handling not implemented for '{event}'"
                    );
                }
            }

            // Forward event to other areas
            self.ready_events.push_back(event);

            // Event got polled and handled
            return true;
        }

        // Did not find or handle any events
        false
    }

    /// Returns a single event that was forwarded to other areas.
    pub fn require_event(&mut self) -> Option<Event> {
        self.ready_events.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_run_correct() {
        let mut engine = Engine::default();
        engine.run().expect("engine should be able to start");
        assert_eq!(engine.state(), EngineState::Running);
    }

    #[test]
    fn engine_stop_correct() {
        let mut engine = Engine::default();
        engine.run().unwrap();
        engine
            .shutdown()
            .expect("engine should be able to shutdown");
        assert_eq!(engine.state(), EngineState::Stopped);
    }
}
