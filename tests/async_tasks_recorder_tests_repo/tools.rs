use std::future::Future;

#[allow(dead_code)]
pub enum RuntimeType {
    CurrentThread,
    CustomThread(usize),
    MultiThread,
}

pub fn do_async_test<Fut>(runtime_type: RuntimeType, test_future: Fut)
    where Fut: Future {
    let runtime;
    match runtime_type {
        RuntimeType::CurrentThread => {
            runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build().unwrap();
        }
        RuntimeType::CustomThread(thread_num) => {
            runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(thread_num)
                .enable_all()
                .build().unwrap();
        }
        RuntimeType::MultiThread => {
            runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build().unwrap();
        }
    }

    runtime.block_on(test_future);
}

/// return a closure to generate incremental `task_id` string.
pub fn get_task_id_generator() -> impl FnMut() -> String {
    let rand_str: String = std::iter::repeat_with(fastrand::alphabetic).take(6).collect();
    let mut count: usize = 0;
    move || {
        let ans = format!("task_id_{}_{:06}", rand_str, count);
        count += 1;
        ans
    }
}

