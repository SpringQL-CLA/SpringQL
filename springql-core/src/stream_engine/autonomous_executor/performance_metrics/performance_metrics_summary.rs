// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::PerformanceMetrics;

/// Summary of [PerformanceMetrics](super::PerformanceMetrics).
///
/// From this summary, [TaskExecutor](crate::stream_processor::autonomous_executor::task_executor::TaskExecutor):
/// - transits memory state diagram
/// - changes task scheduler
/// - launches purger
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(in crate::stream_engine::autonomous_executor) struct PerformanceMetricsSummary {
    pub(in crate::stream_engine::autonomous_executor) queue_total_bytes: u64,
}

impl From<&PerformanceMetrics> for PerformanceMetricsSummary {
    fn from(pm: &PerformanceMetrics) -> Self {
        let queue_total_bytes = Self::queue_total_bytes(pm);
        Self { queue_total_bytes }
    }
}

impl Display for PerformanceMetricsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.queue_total_bytes)
    }
}

impl PerformanceMetricsSummary {
    fn queue_total_bytes(pm: &PerformanceMetrics) -> u64 {
        let window = pm
            .get_window_queues()
            .iter()
            .fold(0, |acc, (_, met)| acc + met.bytes());

        let row = pm
            .get_row_queues()
            .iter()
            .fold(0, |acc, (_, met)| acc + met.bytes());

        window + row
    }
}
