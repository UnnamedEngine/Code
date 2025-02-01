use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam::channel::{Receiver, Sender};
use strum::{Display, EnumCount};
use thiserror::Error;

use super::Job;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// Thread already joined.
    #[error("Thread already joined")]
    ThreadAlreadyJoined,
    /// Failed to join thread.
    #[error("Failed to join thread")]
    ThreadJoinFailure(String),
}

/// All the variants of a `Worker`. Specially useful for defining dedicated
/// workers that will only execute one big `Job`.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, EnumCount, Hash)]
pub enum WorkerKind {
    /// Ccan be used for anything, ideally it should be tasks that can be
    /// executed in parallel.
    /// After a graceful initialization, generic workers can be transformed in
    /// dedicated workers, this can be achieved by using
    /// `WorkerInstruction::Specialize`.
    #[strum(to_string = "Generic({0})")]
    Generic(usize),
    /// Used only for networking.
    Networking,

    /// Only used during tests.
    #[cfg(test)]
    Dummy,
}

/// All the possible states a `Worker` can be at.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    /// Waiting for a new `Job`.
    Idle,
    /// Executing a `Job`.
    Executing,
}

/// All the instructions a `Worker` can receive.
#[derive(Display)]
pub enum WorkerInstruction {
    /// Wait for instructions.
    Wait,
    /// Execute passed `Job`.
    Execute(Job),
    /// End the current `Job` and break the main loop.
    Terminate,
    /// Specialize itself with the passed `Job`.
    Specialize(WorkerKind, Job),
}

#[derive(Debug, Display)]
pub enum WorkerNotification {
    /// Started a job.
    JobStarted(WorkerKind),
    /// Completed a job.
    JobCompleted(WorkerKind),
    /// Started a specialized job and got converted.
    SpecializedJobStarted(WorkerKind),
    /// Completed its specialized job and got converted to generic.
    SpecializedJobCompleted(WorkerKind),
}

/// Wrapper for a `JoinHandle` that contains extra information to help manage
/// the thread.
pub struct Worker {
    /// Instance's kind.
    pub kind: Arc<Mutex<WorkerKind>>,
    /// Instance's current state.
    pub state: Arc<Mutex<WorkerState>>,
    /// This instance's thread handle.
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Receiver<WorkerInstruction>,
        notification_sender: Sender<WorkerNotification>,
    ) -> Self {
        let kind = Arc::new(Mutex::new(WorkerKind::Generic(id)));
        let state = Arc::new(Mutex::new(WorkerState::Idle));

        let kind_clone = Arc::clone(&kind);
        let state_clone = Arc::clone(&state);
        let received_clone = receiver.clone();

        let thread = thread::spawn(move || {
            let kind = *kind_clone.lock().unwrap();
            loop {
                match received_clone.recv() {
                    Ok(instruction) => {
                        match instruction {
                            WorkerInstruction::Wait => {
                                log::warn!("Worker '{kind}' instructed to wait: this is not normal behavior");
                                break;
                            }
                            WorkerInstruction::Execute(job) => {
                                log::info!("Worker '{kind}' instructed to execute a task");

                                {
                                    let mut state = state_clone.lock().unwrap();
                                    *state = WorkerState::Executing;
                                }

                                Self::notify(
                                    &kind,
                                    &notification_sender,
                                    WorkerNotification::JobStarted(kind),
                                );

                                job();

                                {
                                    let mut state = state_clone.lock().unwrap();
                                    *state = WorkerState::Idle;
                                }

                                Self::notify(
                                    &kind,
                                    &notification_sender,
                                    WorkerNotification::JobCompleted(kind),
                                );

                                log::info!("Worker '{kind}' finished the required job and is now idle");
                            }
                            // TODO: find a way to terminate dedicated workers
                            // TODO: allow terminated workers to be reused
                            WorkerInstruction::Terminate => {
                                log::info!(
                                    "Worker '{kind}' instructed to terminate"
                                );
                                break;
                            }
                            WorkerInstruction::Specialize(
                                specialization,
                                job,
                            ) => {
                                let mut kind = kind_clone.lock().unwrap();
                                let kind_copy = *kind;
                                match kind_copy {
                                    WorkerKind::Generic(id) => {
                                        log::info!("Worker '{}' instructed to specialize into '{specialization}'", &*kind);

                                        *kind = specialization;
                                        {
                                            let mut state =
                                                state_clone.lock().unwrap();
                                            *state = WorkerState::Executing;
                                        }

                                        Self::notify(&kind, &notification_sender, WorkerNotification::SpecializedJobStarted(*kind));

                                        job();

                                        // We need to be absolutely sure we
                                        // notify BEFORE changing the kind to
                                        // generic
                                        Self::notify(&kind, &notification_sender, WorkerNotification::SpecializedJobCompleted(*kind));

                                        *kind = WorkerKind::Generic(id);
                                        {
                                            let mut state =
                                                state_clone.lock().unwrap();
                                            *state = WorkerState::Idle;
                                        }

                                        log::info!("Worker '{}' finished the specialized job, got converted back into generic and is now idle", &*kind);
                                    }
                                    _ => {
                                        log::error!("Failed to specialize '{}': only a generic worker can be specialized", &*kind);
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Worker '{kind}' failed to receive instruction: {err}");
                        break;
                    }
                }
            }
        });

        Self {
            kind,
            state,
            thread: Some(thread),
        }
    }

    /// Returns a copy of the current `WorkerKind`.
    pub fn kind(&self) -> WorkerKind {
        *Arc::clone(&self.kind).lock().unwrap()
    }

    /// Returns a copy of the current `WorkerState`.
    pub fn state(&self) -> WorkerState {
        *Arc::clone(&self.state).lock().unwrap()
    }

    /// Joins the thread of this worker.
    pub fn join(&mut self) -> Result<(), Error> {
        if let Some(handle) = self.thread.take() {
            match handle.join() {
                Ok(_) => {
                    log::info!(
                        "Thread from worker '{}' sucessfully joined",
                        self.kind(),
                    );
                    Ok(())
                }
                Err(err) => {
                    if let Some(err_msg) = err.downcast_ref::<&str>() {
                        Err(Error::ThreadJoinFailure(err_msg.to_string()))
                    } else if let Some(err_string) =
                        err.downcast_ref::<String>()
                    {
                        Err(Error::ThreadJoinFailure(err_string.clone()))
                    } else {
                        Err(Error::ThreadJoinFailure(
                            "unknown reason".to_string(),
                        ))
                    }
                }
            }
        } else {
            Err(Error::ThreadAlreadyJoined)
        }
    }

    fn notify(
        kind: &WorkerKind,
        notification_sender: &Sender<WorkerNotification>,
        notification: WorkerNotification,
    ) {
        if let Err(err) = notification_sender.send(notification) {
            log::error!(
                "Failed to send notification from worker '{kind}': {err}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        time::{Duration, Instant},
    };

    use crossbeam::channel::unbounded;
    use serial_test::serial;

    use super::*;

    #[derive(Debug, PartialEq, Eq, Display, Hash)]
    enum WorkerNotificationVariant {
        JobStarted,
        JobCompleted,
        SpecializedJobStarted,
        SpecializedJobCompleted,
    }

    fn wait_for_state<F>(
        condition: F,
        timeout: Duration,
        check_interval: Duration,
    ) -> bool
    where
        F: Fn() -> bool,
    {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if condition() {
                return true;
            }
            thread::sleep(check_interval);
        }
        false
    }

    fn verify_absolute_state(
        workers: &Vec<Worker>,
        desired_state: WorkerState,
        expected_notifications: Vec<(WorkerNotificationVariant, usize)>,
        notification_receiver: Receiver<WorkerNotification>,
    ) -> bool {
        // Wait some time just to be sure
        thread::sleep(Duration::from_millis(16));

        // Verify if workers are in desired state
        for worker in workers {
            let state_check = wait_for_state(
                || *worker.state.lock().unwrap() == desired_state,
                Duration::from_secs(1),
                Duration::from_millis(16),
            );
            if !state_check {
                println!(
                    "Worker '{:?}' did not enter '{}' state before timeout",
                    worker.kind.lock().unwrap(),
                    desired_state,
                );
                return false;
            }
        }

        // Prepare a map to hold counts of expected notifications
        let mut expected_counts = HashMap::new();
        for (variant, count) in expected_notifications {
            expected_counts.insert(variant, count);
        }

        // Collect notifications until we've received the expected counts
        let start = Instant::now();
        while !expected_counts.is_empty()
            && start.elapsed() < Duration::from_secs(2)
        {
            match notification_receiver.try_recv() {
                Ok(notification) => {
                    // Determine the variant of the notification
                    let variant = match notification {
                        WorkerNotification::JobStarted(_) => {
                            WorkerNotificationVariant::JobStarted
                        }
                        WorkerNotification::JobCompleted(_) => {
                            WorkerNotificationVariant::JobCompleted
                        }
                        WorkerNotification::SpecializedJobStarted(_) => {
                            WorkerNotificationVariant::SpecializedJobStarted
                        }
                        WorkerNotification::SpecializedJobCompleted(_) => {
                            WorkerNotificationVariant::SpecializedJobCompleted
                        }
                    };

                    // If this variant is one we're expecting, decrement its count
                    if let Some(count) = expected_counts.get_mut(&variant) {
                        if *count > 0 {
                            *count -= 1;
                            if *count == 0 {
                                expected_counts.remove(&variant);
                            }
                        }
                    } else {
                        // We received an unexpected notification
                        println!(
                            "Unexpected notification received: '{:?}'",
                            notification,
                        );
                        return false;
                    }
                }
                Err(_) => {
                    // If no notifications are available, wait a bit
                    thread::sleep(Duration::from_millis(16));
                }
            }
        }

        if !expected_counts.is_empty() {
            println!(
                "Did not receive the expected number of notifications before timeout: {:?}",
                expected_counts,
            );
            return false;
        }

        true
    }

    #[test]
    #[serial]
    fn create_worker() {
        let (_, receiver) = unbounded();
        let (notification_sender, _) = unbounded();

        let mut worker = Worker::new(0, receiver.clone(), notification_sender);

        assert_eq!(*worker.kind.lock().unwrap(), WorkerKind::Generic(0));
        assert_eq!(*worker.state.lock().unwrap(), WorkerState::Idle);

        let _ = worker.join();
    }

    #[test]
    #[serial]
    fn worker_execute() {
        let (sender, receiver) = unbounded();
        let (notification_sender, notification_receiver) = unbounded();

        let workers = (0..1)
            .map(|id| {
                Worker::new(id, receiver.clone(), notification_sender.clone())
            })
            .collect::<Vec<_>>();

        for _ in &workers {
            let _ = sender.send(WorkerInstruction::Execute(Box::new(|| {
                thread::sleep(Duration::from_millis(100));
            })));
        }

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Executing,
            vec![(WorkerNotificationVariant::JobStarted, 1)],
            notification_receiver.clone()
        ),);

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Idle,
            vec![(WorkerNotificationVariant::JobCompleted, 1)],
            notification_receiver.clone()
        ),);

        // Terminate the workers
        for _ in &workers {
            let _ = sender.send(WorkerInstruction::Terminate);
        }

        for mut worker in workers {
            let _ = worker.join();
        }
    }

    #[test]
    #[serial]
    fn worker_execute_multiple() {
        let (sender, receiver) = unbounded();
        let (notification_sender, notification_receiver) = unbounded();

        let workers = (0..4)
            .map(|id| {
                Worker::new(id, receiver.clone(), notification_sender.clone())
            })
            .collect::<Vec<_>>();

        for _ in &workers {
            let _ = sender.send(WorkerInstruction::Execute(Box::new(|| {
                thread::sleep(Duration::from_millis(100));
            })));
        }

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Executing,
            vec![(WorkerNotificationVariant::JobStarted, 4)],
            notification_receiver.clone()
        ),);

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Idle,
            vec![(WorkerNotificationVariant::JobCompleted, 4)],
            notification_receiver.clone()
        ),);

        // Terminate the workers
        for _ in &workers {
            let _ = sender.send(WorkerInstruction::Terminate);
        }

        for mut worker in workers {
            let _ = worker.join();
        }
    }

    #[test]
    #[serial]
    fn worker_execute_more_than_available() {
        let (sender, receiver) = unbounded();
        let (notification_sender, notification_receiver) = unbounded();

        let workers = (0..4)
            .map(|id| {
                Worker::new(id, receiver.clone(), notification_sender.clone())
            })
            .collect::<Vec<_>>();

        let total_jobs = 8;

        // Send double the amount of jobs
        for _ in 0..total_jobs {
            let _ = sender.send(WorkerInstruction::Execute(Box::new(|| {
                thread::sleep(Duration::from_millis(100));
            })));
        }

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Executing,
            vec![(WorkerNotificationVariant::JobStarted, 4)],
            notification_receiver.clone()
        ),);

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Idle,
            vec![
                (WorkerNotificationVariant::JobStarted, 4),
                (WorkerNotificationVariant::JobCompleted, 4),
            ],
            notification_receiver.clone()
        ),);

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Idle,
            vec![(WorkerNotificationVariant::JobCompleted, 4)],
            notification_receiver.clone()
        ),);

        // Terminate the workers
        for _ in &workers {
            let _ = sender.send(WorkerInstruction::Terminate);
        }

        for mut worker in workers {
            let _ = worker.join();
        }
    }

    #[test]
    #[serial]
    fn execute_dedicated() {
        let (sender, receiver) = unbounded();
        let (notification_sender, notification_receiver) = unbounded();

        let workers = (0..1)
            .map(|id| {
                Worker::new(id, receiver.clone(), notification_sender.clone())
            })
            .collect::<Vec<_>>();

        for _ in &workers {
            let _ = sender.send(WorkerInstruction::Specialize(
                WorkerKind::Dummy,
                Box::new(|| {
                    thread::sleep(Duration::from_millis(100));
                }),
            ));
        }

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Executing,
            vec![(WorkerNotificationVariant::SpecializedJobStarted, 1)],
            notification_receiver.clone()
        ),);

        assert!(verify_absolute_state(
            &workers,
            WorkerState::Idle,
            vec![(WorkerNotificationVariant::SpecializedJobCompleted, 1)],
            notification_receiver.clone()
        ),);

        // Terminate the workers
        for _ in &workers {
            let _ = sender.send(WorkerInstruction::Terminate);
        }

        for mut worker in workers {
            let _ = worker.join();
        }
    }
}
