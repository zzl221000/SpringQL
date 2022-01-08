use std::{
    sync::{mpsc, Arc},
    thread,
};

use crate::{
    error::SpringError, stream_engine::autonomous_executor::performance_metrics::PerformanceMetrics,
};

use crate::stream_engine::autonomous_executor::{
    event_queue::{
        event::{Event, EventTag},
        EventPoll, EventQueue,
    },
    pipeline_derivatives::PipelineDerivatives,
};

pub(in crate::stream_engine::autonomous_executor) trait WorkerThread {
    /// Immutable argument to pass to `main_loop_cycle`.
    ///
    /// Use `()` if no arg needed.
    type ThreadArg: Send + 'static;

    /// State updated in each `main_loop_cycle`.
    ///
    /// Use `()` if no state needed.
    type LoopState: Default;

    /// Which events to subscribe
    fn event_subscription() -> Vec<EventTag>;

    /// A cycle in `main_loop`
    fn main_loop_cycle(
        current_state: Self::LoopState,
        thread_arg: &Self::ThreadArg,
    ) -> Self::LoopState;

    fn ev_update_pipeline(
        current_state: Self::LoopState,
        pipeline_derivatives: Arc<PipelineDerivatives>,
        thread_arg: &Self::ThreadArg,

        // for cascading event
        event_queue: Arc<EventQueue>,
    ) -> Self::LoopState;

    fn ev_update_performance_metrics(
        current_state: Self::LoopState,
        metrics: Arc<PerformanceMetrics>,
        thread_arg: &Self::ThreadArg,

        // for cascading event
        event_queue: Arc<EventQueue>,
    ) -> Self::LoopState;

    /// Worker thread's entry point
    fn run(
        event_queue: Arc<EventQueue>,
        stop_receiver: mpsc::Receiver<()>,
        thread_arg: Self::ThreadArg,
    ) {
        let event_polls = Self::event_subscription()
            .into_iter()
            .map(|ev| event_queue.subscribe(ev))
            .collect();
        let _ = thread::spawn(move || {
            Self::main_loop(event_queue, event_polls, stop_receiver, thread_arg)
        });
    }

    fn main_loop(
        event_queue: Arc<EventQueue>,
        event_polls: Vec<EventPoll>,
        stop_receiver: mpsc::Receiver<()>,
        thread_arg: Self::ThreadArg,
    ) {
        let mut state = Self::LoopState::default();

        while stop_receiver.try_recv().is_err() {
            state = Self::main_loop_cycle(state, &thread_arg);
            state = Self::handle_events(state, &event_polls, &thread_arg, event_queue.clone());
        }
    }

    fn handle_events(
        current_state: Self::LoopState,
        event_polls: &[EventPoll],
        thread_arg: &Self::ThreadArg,
        event_queue: Arc<EventQueue>,
    ) -> Self::LoopState {
        let mut state = current_state;

        for event_poll in event_polls {
            #[allow(clippy::single_match)]
            match event_poll.poll() {
                Some(Event::UpdatePipeline {
                    pipeline_derivatives,
                }) => {
                    state = Self::ev_update_pipeline(
                        state,
                        pipeline_derivatives,
                        thread_arg,
                        event_queue.clone(),
                    );
                }
                Some(Event::UpdatePerformanceMetrics { metrics }) => {
                    state = Self::ev_update_performance_metrics(
                        state,
                        metrics,
                        thread_arg,
                        event_queue.clone(),
                    );
                }
                None => {}
            }
        }
        state
    }

    fn handle_error(e: SpringError) {
        match e {
            SpringError::ForeignSourceTimeout { .. } | SpringError::InputTimeout { .. } => {
                log::trace!("{:?}", e)
            }

            SpringError::ForeignIo { .. }
            | SpringError::SpringQlCoreIo(_)
            | SpringError::Unavailable { .. } => log::warn!("{:?}", e),

            SpringError::InvalidOption { .. }
            | SpringError::InvalidFormat { .. }
            | SpringError::Sql(_)
            | SpringError::ThreadPoisoned(_) => log::error!("{:?}", e),
        }
    }
}
