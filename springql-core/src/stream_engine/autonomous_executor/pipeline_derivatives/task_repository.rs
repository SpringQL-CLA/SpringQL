// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::{borrow::BorrowMut, collections::HashMap, sync::Arc};

use anyhow::anyhow;

use crate::{
    error::{Result, SpringError},
    pipeline::pipeline_graph::PipelineGraph,
    stream_engine::autonomous_executor::{
        task::{window::Window, Task},
        task_graph::task_id::TaskId,
    },
};

#[derive(Debug, Default)]
pub(in crate::stream_engine::autonomous_executor) struct TaskRepository {
    repo: HashMap<TaskId, Arc<Task>>,
}

impl TaskRepository {
    /// # Failure
    ///
    /// - [SpringError::Sql](crate::error::SpringError::Sql) when:
    ///   - TaskId is not found in task repo.
    pub(super) fn get(&self, task_id: &TaskId) -> Result<Arc<Task>> {
        self.repo
            .get(task_id)
            .ok_or_else(|| {
                SpringError::Sql(anyhow!("task id {} is not in TaskRepository", task_id))
            })
            .map(|t| t.clone())
    }

    pub(in crate::stream_engine::autonomous_executor) fn purge_windows(&self) {
        for task in self.repo.values() {
            if let Task::Pump(pump_task) = task.as_ref() {
                if let Some(mut w) = pump_task.get_join_window_mut() {
                    w.borrow_mut().purge()
                } else if let Some(mut w) = pump_task.get_aggr_window_mut() {
                    w.borrow_mut().purge()
                } else {
                    // nothing to purge
                }
            }
        }
    }
}

impl From<&PipelineGraph> for TaskRepository {
    fn from(pipeline_graph: &PipelineGraph) -> Self {
        let pipeline_petgraph = pipeline_graph.as_petgraph();
        let repo = pipeline_petgraph
            .edge_weights()
            .map(|edge| {
                let task = Task::new(edge, pipeline_graph);
                (task.id(), Arc::new(task))
            })
            .collect();
        Self { repo }
    }
}
