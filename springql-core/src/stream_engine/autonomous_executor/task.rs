// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

pub(super) mod pump_task;
pub(super) mod task_context;
pub(super) mod task_graph;
pub(super) mod task_id;
pub(super) mod task_state;

mod sink_task;
mod source_task;
mod subtask;

use crate::{error::Result, stream_engine::dependency_injection::DependencyInjection};

use self::{
    pump_task::PumpTask, sink_task::SinkTask, source_task::SourceTask, task_context::TaskContext,
    task_id::TaskId, task_state::TaskState,
};

#[derive(Debug)]
pub(crate) enum Task {
    Pump(PumpTask),
    Source(SourceTask),
    Sink(SinkTask),
}

impl Task {
    pub(super) fn id(&self) -> TaskId {
        match self {
            Task::Pump(t) => t.id().clone(),
            Task::Source(source_task) => source_task.id().clone(),
            Task::Sink(s) => s.id().clone(),
        }
    }

    pub(super) fn state(&self) -> TaskState {
        match self {
            Task::Pump(t) => t.state().clone(),
            Task::Source(source_task) => source_task.state().clone(),
            Task::Sink(s) => s.state(),
        }
    }

    pub(super) fn run<DI: DependencyInjection>(&self, context: &TaskContext<DI>) -> Result<()> {
        match self {
            Task::Pump(pump_task) => pump_task.run(context),
            Task::Source(source_task) => source_task.run(context),
            Task::Sink(sink_task) => sink_task.run(context),
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl Eq for Task {}