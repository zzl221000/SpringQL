// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

pub(in crate::stream_engine::autonomous_executor) mod flow_efficient_scheduler;

use std::fmt::Debug;
use std::sync::Arc;

use crate::error::Result;
use crate::pipeline::pipeline_version::PipelineVersion;
use crate::stream_engine::autonomous_executor::current_pipeline::CurrentPipeline;
use crate::stream_engine::autonomous_executor::task::task_graph::TaskGraph;
use crate::stream_engine::autonomous_executor::task::Task;

pub(crate) trait WorkerState {}

pub(crate) trait Scheduler: Debug + Default + Sync + Send + 'static {
    type W: WorkerState + Clone + Default;

    /// Called from main thread.
    fn notify_pipeline_update(&mut self, current_pipeline: &CurrentPipeline) -> Result<()> {
        let inner = current_pipeline.read();
        let pipeline = inner.pipeline();
        let task_graph = inner.task_graph();
        self._notify_pipeline_version(pipeline.version());
        self._notify_task_graph_update(task_graph)
    }

    /// Called from main thread.
    fn _notify_pipeline_version(&mut self, v: PipelineVersion);

    /// Called from main thread.
    fn _notify_task_graph_update(&mut self, task_graph: &TaskGraph) -> Result<()>;

    /// Called from worker threads.
    fn next_task(&self, worker_state: Self::W) -> Option<(Arc<Task>, Self::W)>;
}
