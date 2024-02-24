#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TaskState {
    /// running or pending
    Working,
    Success,
    Failed,
    NotFound,
    Revoking,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum RevokeFailReason<Fut, E>
    where Fut: Send,
          E: Send {
    NotSuccess(TaskState, Fut),
    RevokeTaskError(E),
}
