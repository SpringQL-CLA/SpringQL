// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

pub(in crate::stream_engine::autonomous_executor) mod source_worker_id;
pub(in crate::stream_engine::autonomous_executor) mod source_worker_thread;

use std::sync::Arc;

use crate::stream_engine::autonomous_executor::{
    event_queue::EventQueue, worker::worker_handle::WorkerHandle,
};

use self::source_worker_thread::{SourceWorkerThread, SourceWorkerThreadArg};

/// Worker to execute pump and sink tasks.
#[derive(Debug)]
pub(super) struct SourceWorker {
    handle: WorkerHandle,
}

impl SourceWorker {
    pub(super) fn new(event_queue: Arc<EventQueue>, thread_arg: SourceWorkerThreadArg) -> Self {
        let handle = WorkerHandle::new::<SourceWorkerThread>(event_queue, thread_arg);
        Self { handle }
    }
}
