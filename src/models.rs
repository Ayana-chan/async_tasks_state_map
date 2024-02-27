#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TaskState {
    /// Running or pending.
    Working,
    Success,
    Failed,
    /// Never appear in map, only returned by query when the target task is not in map.
    NotFound,
    Revoking,
}
