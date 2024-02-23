#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TaskState {
    /// running or pending
    Working,
    Success,
    Failed,
    NotFound,
}
