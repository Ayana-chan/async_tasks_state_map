use std::borrow::Borrow;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use crate::*;

// TODO T可能不需要clone

#[derive(Debug, Clone)]
pub struct AsyncTasksRecorder<T>
    where T: Eq + Hash + Clone + Send + Sync + 'static {
    recorder: Arc<scc::HashMap<T, TaskState>>,
}

/// Public interfaces.
impl<K> AsyncTasksRecorder<K>
    where K: Eq + Hash + Clone + Send + Sync + 'static {
    /// Create a completely new `AsyncTasksRecoder`.
    pub fn new() -> Self {
        AsyncTasksRecorder {
            recorder: scc::HashMap::new().into(),
        }
    }

    /// Create by a map.
    pub fn new_with_task_manager(recorder: scc::HashMap<K, TaskState>) -> Self {
        AsyncTasksRecorder {
            recorder: recorder.into(),
        }
    }

    /// Create by an `Arc` of map.
    pub fn new_with_task_manager_arc(recorder: Arc<scc::HashMap<K, TaskState>>) -> Self {
        AsyncTasksRecorder {
            recorder,
        }
    }

    /// Launch a task and execute it asynchronously
    pub async fn launch<Fut, R, E>(&self, task_id: K, task: Fut) -> Result<(), (TaskState, Fut)>
        where Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        let mut launch_flag = None;

        self.recorder.entry_async(task_id.clone()).await
            .and_modify(|v| {
                if *v != TaskState::Failed {
                    launch_flag = Some(v.clone());
                    return;
                }
                *v = TaskState::Working;
            })
            // not found
            .or_insert_with(|| {
                TaskState::Working
            });

        if let Some(reason) = launch_flag {
            return Err((reason, task));
        }

        // start
        let recorder = self.recorder.clone();
        tokio::spawn(async move {
            Self::launch_task_fut(&recorder, task_id, task).await;
        });

        Ok(())
    }

    /// Launch a task. Not return (keep awaiting) until the task finishes.
    pub async fn launch_block<Fut, R, E>(&self, task_id: K, task: Fut) -> Result<(), (TaskState, Fut)>
        where Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        let mut launch_flag = None;

        self.recorder.entry_async(task_id.clone()).await
            .and_modify(|v| {
                if *v != TaskState::Failed {
                    launch_flag = Some(v.clone());
                    return;
                }
                *v = TaskState::Working;
            })
            // not found
            .or_insert_with(|| {
                TaskState::Working
            });

        if let Some(reason) = launch_flag {
            return Err((reason, task));
        }

        // start (block)
        let recorder = self.recorder.clone();
        Self::launch_task_fut(&recorder, task_id, task).await;

        Ok(())
    }

    /// Query the target task's state.
    pub async fn query_task_state<Q>(&self, task_id: &Q) -> TaskState
        where K: Borrow<Q>,
              Q: Hash + Eq + ?Sized {
        let res = self.recorder.get_async(task_id).await;
        match res {
            Some(res) => res.get().clone(),
            None => TaskState::NotFound,
        }
    }

    /// Revoke target task with its `task_id` and a `Future` for revoking.
    pub async fn revoke_task_block<Q, Fut, R, E>(&self, target_task_id: &Q, revoke_task: Fut) -> Result<R, RevokeFailReason<Fut, E>>
        where K: Borrow<Q>,
              Q: Hash + Eq + ?Sized,
              Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        let ent = self.recorder.get_async(target_task_id).await;
        match ent {
            Some(mut ent) => {
                let state = ent.get_mut();
                if *state != TaskState::Success {
                    return Err(RevokeFailReason::NotSuccess(state.clone(), revoke_task));
                }
                *state = TaskState::Revoking;
            }
            None => return Err(RevokeFailReason::NotSuccess(TaskState::NotFound, revoke_task)),
        };

        // start to revoke
        let revoke_res = revoke_task.await;
        match revoke_res {
            Ok(r) => {
                self.recorder.remove_async(target_task_id).await;
                Ok(r)
            }
            Err(e) => {
                self.recorder.update_async(target_task_id,
                                           |_, v| *v = TaskState::Success).await;
                Err(RevokeFailReason::RevokeTaskError(e))
            }
        }
    }

    /// Get a reference of the internal map.
    pub fn get_recorder_ref(&self) -> &scc::HashMap<K, TaskState> {
        &self.recorder
    }
}

/// Private tools.
impl<T> AsyncTasksRecorder<T>
    where T: Eq + Hash + Clone + Send + Sync + 'static {
    /// The async function to execute launched tasks.
    async fn launch_task_fut<Fut, R, E>(
        recorder: &scc::HashMap<T, TaskState>,
        task_id: T, task: Fut)
        where Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        // execute task
        let task_res = task.await;

        // handle result
        if task_res.is_ok() {
            recorder.update_async(
                &task_id,
                |_, v| *v = TaskState::Success)
                .await.unwrap();
        } else {
            recorder.update_async(
                &task_id,
                |_, v| *v = TaskState::Failed)
                .await.unwrap();
        }
    }
}
