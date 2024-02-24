# Introduction

A struct for recording execution status of async tasks with async methods.

Functions:
- Able to host `Future`s and query whether they are
  **not found**, **running**, **successful**, **failed**, or **revoking**.
- Able to host `Future`s to revoke the succeeded `Future`s and make them **not found**.

Dependency:
- Depend on `tokio` with feature `rt`, so cannot use other async runtimes.
- Depend on [scc](https://crates.io/crates/scc) for async `HashMap`.

Use this crate if:
- Easy to generate an **unique** `task_id` (not necessarily `String`) for a future (task).
- Don't want tasks with the same `task_id` to succeed more than once.
- Require **linearizability**.
- Want to revoke a task, and don't want the revoking to succeed more than once.

A recorder can only use **single** `task_id` type. The type of `task_id` should be:
- `Eq + Hash + Clone + Send + Sync + 'static`
- Cheap to clone (sometimes can use `Arc`) (only cloned once when launch).

> [async_tasks_recorder](https://crates.io/crates/async_tasks_recorder)
is another implement depending on `HashSet`,
which is easier to iterate every task in the same state.
But you should not use that crate if you only focus on iterating only one state.
Instead, you can collect the tasks in certain state into an external `Arc<HashSet>`.

# State Transition Diagram

```txt
    ┌------- Revoking ←-----┐
    ↓                       |
NotFound --> Working --> Success
               ↑ |
               | ↓
             Failed
```

- Can only launch when `NotFound` or `Failed`.
- Can only revoke when `Success`.

# Advices

## Simplified State Transition Diagram

- In some cases, `NotFound` can be equivalent to `Failed`.
- In most cases, `Revoking` can be equivalent to `Success`.

So you may get:
```txt
    ┌----------------------┐
    ↓                      |
Failed <--> Working --> Success
```
