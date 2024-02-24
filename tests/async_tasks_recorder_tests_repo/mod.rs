use async_tasks_state_map::*;

mod tools;

pub use tools::{RuntimeType, do_async_test};

pub async fn test_simple_launch_check(task_num: usize) {
    let manager = AsyncTasksRecorder::new();
    let mut task_id_generator = tools::get_task_id_generator();

    let mut join_set = tokio::task::JoinSet::new();
    for _ in 0..task_num {
        let manager = manager.clone();
        let task_id = task_id_generator();
        // let task_id_backup = task_id.clone();
        let task = async move {
            // println!("task start {}", task_id_backup);
            let latency = fastrand::u64(5..30);
            tokio::time::sleep(tokio::time::Duration::from_millis(latency)).await;
            // println!("task finish {}", task_id_backup);
            Ok::<(), ()>(())
        };

        join_set.spawn(async move {
            // launch
            assert_eq!(manager.query_task_state(&task_id).await, TaskState::NotFound,
                       "Initial state should be NotFound {}", task_id);
            let res = manager.launch(task_id.clone(), task).await;
            assert!(res.is_ok(),
                    "Launch should success {}", task_id);
            assert_ne!(manager.query_task_state(&task_id).await, TaskState::NotFound,
                       "Shouldn't be NotFound after launch {}", task_id);
            assert_ne!(manager.query_task_state(&task_id).await, TaskState::Failed,
                       "Shouldn't be Failed after launch {}", task_id);

            // revoke
            loop {
                match manager.query_task_state(&task_id).await {
                    TaskState::Success => {
                        // finish
                        // println!("Judge success {}", task_id);
                        return;
                    }
                    TaskState::Working => {
                        // wait for a little random time
                        let wait_time = fastrand::u64(1..50);
                        tokio::time::sleep(tokio::time::Duration::from_micros(wait_time)).await;
                    }
                    state => {
                        panic!("Unexpected task state {}: {:?}", task_id, state);
                    }
                }
            }
        });
    }

    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            if e.is_panic() {
                std::panic::resume_unwind(e.into_panic());
            }
        }
    }
}

pub async fn test_simple_launch_check_revoke_loop(task_num: usize, redo_num: usize) {
    let manager = AsyncTasksRecorder::new();
    let mut task_id_generator = tools::get_task_id_generator();

    let mut join_set = tokio::task::JoinSet::new();
    for _ in 0..task_num {
        let manager = manager.clone();
        let task_id = task_id_generator();

        join_set.spawn(async move {
            for _ in 0..redo_num {
                // println!("redo: {}", task_id);
                // let task_id_backup = task_id.clone();
                let task = async move {
                    // println!("task start {}", task_id_backup);
                    tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
                    // println!("task finish {}", task_id_backup);
                    Ok::<(), ()>(())
                };

                // launch
                assert_eq!(manager.query_task_state(&task_id).await, TaskState::NotFound,
                           "Initial state should be NotFound {}", task_id);
                let res = manager.launch(task_id.clone(), task).await;
                assert!(res.is_ok(),
                        "Launch should success {}", task_id);
                assert_ne!(manager.query_task_state(&task_id).await, TaskState::NotFound,
                           "Shouldn't be NotFound after launch {}", task_id);
                assert_ne!(manager.query_task_state(&task_id).await, TaskState::Failed,
                           "Shouldn't be Failed after launch {}", task_id);

                // revoke
                'check_success: loop {
                    match manager.query_task_state(&task_id).await {
                        TaskState::Success => {
                            // revoke. only once
                            // let task_id_backup = task_id.clone();
                            let revoke_task = async move {
                                // println!("revoke task start {}", task_id_backup);
                                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                                // println!("revoke task finish {}", task_id_backup);
                                Ok::<(), ()>(())
                            };
                            let res = manager.revoke_task_block(&task_id, revoke_task).await;
                            assert!(res.is_ok());
                            assert_eq!(manager.query_task_state(&task_id).await, TaskState::NotFound);

                            // println!("Judge success {}", task_id);
                            // wait for a little time
                            tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
                            // finish
                            break 'check_success;
                        }
                        TaskState::Working => {
                            // wait for a little time
                            tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
                        }
                        state => {
                            panic!("Unexpected task state {}: {:?}", task_id, state);
                        }
                    }
                }
            }
        });
    }

    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            if e.is_panic() {
                std::panic::resume_unwind(e.into_panic());
            }
        }
    }
}
