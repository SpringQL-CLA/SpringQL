// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

pub(in crate::stream_engine::autonomous_executor) mod sink_writer;

use std::sync::Arc;

use super::task_context::TaskContext;
use crate::error::Result;
use crate::mem_size::MemSize;
use crate::pipeline::name::{SinkWriterName, StreamName};
use crate::pipeline::sink_writer_model::SinkWriterModel;
use crate::stream_engine::autonomous_executor::performance_metrics::metrics_update_command::metrics_update_by_task_execution::{MetricsUpdateByTaskExecution, TaskMetricsUpdateByTask, InQueueMetricsUpdateByTask, InQueueMetricsUpdateByCollect};
use crate::stream_engine::autonomous_executor::repositories::Repositories;
use crate::stream_engine::autonomous_executor::row::foreign_row::sink_row::SinkRow;
use crate::stream_engine::autonomous_executor::row::Row;
use crate::stream_engine::autonomous_executor::task_graph::queue_id::QueueId;
use crate::stream_engine::autonomous_executor::task_graph::task_id::TaskId;
use crate::stream_engine::time::duration::wall_clock_duration::wall_clock_stopwatch::WallClockStopwatch;

#[derive(Debug)]
pub(crate) struct SinkTask {
    id: TaskId,
    upstream: StreamName,
    sink_writer_name: SinkWriterName,
}

impl SinkTask {
    pub(in crate::stream_engine) fn new(sink_writer: &SinkWriterModel) -> Self {
        let id = TaskId::from_sink(sink_writer);
        Self {
            id,
            upstream: sink_writer.from_sink_stream().clone(),
            sink_writer_name: sink_writer.name().clone(),
        }
    }

    pub(in crate::stream_engine::autonomous_executor) fn id(&self) -> &TaskId {
        &self.id
    }

    pub(in crate::stream_engine::autonomous_executor) fn run(
        &self,
        context: &TaskContext,
    ) -> Result<MetricsUpdateByTaskExecution> {
        let stopwatch = WallClockStopwatch::start();

        let repos = context.repos();

        let queue_id = context
            .pipeline_derivatives()
            .task_graph()
            .input_queue(&context.task(), &self.upstream);

        let in_queues_metrics =
            if let Some((row, in_queue_metrics)) = self.use_row_from(queue_id, repos) {
                self.emit(row, context)?;
                vec![in_queue_metrics]
            } else {
                vec![]
            };

        let execution_time = stopwatch.stop();

        let out_queues_metrics = vec![];
        let task_metrics = TaskMetricsUpdateByTask::new(context.task(), execution_time);
        Ok(MetricsUpdateByTaskExecution::new(
            task_metrics,
            in_queues_metrics,
            out_queues_metrics,
        ))
    }

    fn use_row_from(
        &self,
        queue_id: QueueId,
        repos: Arc<Repositories>,
    ) -> Option<(Row, InQueueMetricsUpdateByTask)> {
        match queue_id {
            QueueId::Row(queue_id) => {
                let row_q_repo = repos.row_queue_repository();
                let queue = row_q_repo.get(&queue_id);
                queue.use_().map(|row| {
                    let bytes_used = row.mem_size();
                    (
                        row,
                        InQueueMetricsUpdateByTask::new(
                            InQueueMetricsUpdateByCollect::Row {
                                queue_id,
                                rows_used: 1,
                                bytes_used: bytes_used as u64,
                            },
                            None,
                        ),
                    )
                })
            }
            QueueId::Window(_) => unreachable!("sink task must have row input queue"),
        }
    }

    fn emit(&self, row: Row, context: &TaskContext) -> Result<()> {
        let f_row = SinkRow::from(row);

        let sink_writer = context
            .repos()
            .sink_writer_repository()
            .get_sink_writer(&self.sink_writer_name);

        sink_writer
            .lock()
            .expect("other worker threads sharing the same sink subtask must not get panic")
            .send_row(f_row)?;

        Ok(())
    }
}
