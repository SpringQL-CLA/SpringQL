// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use parking_lot::RwLock;

use crate::stream_engine::autonomous_executor::task_graph::queue_id::window_queue_id::WindowQueueId;

use super::window_queue::WindowQueue;

#[derive(Debug, Default)]
pub(in crate::stream_engine::autonomous_executor) struct WindowQueueRepository {
    repo: RwLock<HashMap<WindowQueueId, Arc<WindowQueue>>>,
}

impl WindowQueueRepository {
    pub(in crate::stream_engine::autonomous_executor) fn get(
        &self,
        window_queue_id: &WindowQueueId,
    ) -> Arc<WindowQueue> {
        let repo = self.repo.read();
        repo.get(window_queue_id)
            .unwrap_or_else(|| {
                panic!(
                    "Window queue id {} is not in WindowQueueRepository",
                    window_queue_id
                )
            })
            .clone()
    }

    /// Removes all currently existing queues and creates new empty ones.
    pub(in crate::stream_engine::autonomous_executor) fn reset(
        &self,
        queue_ids: HashSet<WindowQueueId>,
    ) {
        let mut repo = self.repo.write();
        repo.clear();

        queue_ids.into_iter().for_each(|queue_id| {
            repo.insert(queue_id, Arc::new(WindowQueue::default()));
        });
    }

    pub(in crate::stream_engine::autonomous_executor) fn purge(&self) {
        let mut repo = self.repo.write();
        repo.iter_mut().for_each(|(_, queue)| {
            queue.purge();
        });
    }
}
