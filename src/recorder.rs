use std::borrow::Borrow;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use crate::*;

#[derive(Debug, Clone)]
pub struct AsyncTasksRecorder<T>
    where T: Eq + Hash + Clone + Send + 'static {
    recorder: Arc<scc::HashMap<T, TaskState>>,
}

impl<T> AsyncTasksRecorder<T>
    where T: Eq + Hash + Clone + Send + 'static {
    /// Create a completely new `AsyncTasksRecoder`.
    pub async fn new() -> Self {
        AsyncTasksRecorder {
            recorder: scc::HashMap::new().into(),
        }
    }

    /// Create by a map.
    pub fn new_with_task_manager(recorder: scc::HashMap<T, TaskState>) -> Self {
        AsyncTasksRecorder {
            recorder: recorder.into(),
        }
    }

    /// Create by an `Arc` of map.
    pub fn new_with_task_manager_arc(recorder: Arc<scc::HashMap<T, TaskState>>) -> Self {
        AsyncTasksRecorder {
            recorder,
        }
    }

    pub async fn launch<Fut, R, E>(&self, task_id: T, task: Fut) -> Result<(), (TaskState, Fut)>
        where Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        todo!()
    }

    pub async fn launch_block<Fut, R, E>(&self, task_id: T, task: Fut) -> Result<(), (TaskState, Fut)>
        where Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        todo!()
    }

    pub async fn query_task_state<Q>(&self, task_id: &Q) -> TaskState
        where T: Borrow<Q>,
              Q: Hash + Eq + ?Sized {
        todo!()
    }

    pub async fn revoke_task_block<Fut, R, E>(&self, target_task_id: T, revoke_task: Fut) -> Result<R, RevokeFailReason<Fut, E>>
        where Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        todo!()
    }

    /// Get a reference of the internal map.
    pub fn get_recorder_ref(&self) -> &scc::HashMap<T, TaskState> {
        &self.recorder
    }
}
