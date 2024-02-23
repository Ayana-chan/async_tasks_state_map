use std::hash::Hash;
use std::sync::Arc;
use crate::*;

pub struct AsyncTasksRecorder<T>
    where T: Eq + Hash + Clone + Send + 'static {
    recorder: Arc<scc::HashMap<T, TaskState>>,
}
