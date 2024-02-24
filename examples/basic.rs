#![allow(dead_code, unused_variables)]

use std::sync::Arc;
use async_tasks_state_map::{AsyncTasksRecorder, TaskState};

struct SimulatedStream {}

struct UploadFileArgs {
    /// file stream
    stream: SimulatedStream,
    md5: String,
}

#[derive(Debug, Eq, PartialEq)]
enum UploadTaskState {
    Uploading,
    Success,
    Failed,
    NotFound,
    Revoking,
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(simulate_requests())
}

/// Simulate front-end request.
async fn simulate_requests() {
    println!("hello world!");
    let recorder = AsyncTasksRecorder::new();
    let fake_md5 = "d8q793wye1u3".to_string();

    println!("REQUEST: check_upload_state {}", fake_md5);
    let result = check_upload_state(
        recorder.clone(),
        fake_md5.to_string(),
    ).await;
    assert_eq!(result, UploadTaskState::NotFound);
    println!("RESPONSE: check_upload_state {}: {:?}", fake_md5, result);

    println!("REQUEST: upload_file {}", fake_md5);
    upload_file(recorder.clone(),
                UploadFileArgs {
                    stream: SimulatedStream {},
                    md5: fake_md5.clone(),
                },
    ).await;

    println!("REQUEST: check_upload_state {}", fake_md5);
    let result = check_upload_state(
        recorder.clone(),
        fake_md5.to_string(),
    ).await;
    // Because the state is queried after `launch().await`, here must be `Uploading` (Too early to be `Success`).
    assert_eq!(result, UploadTaskState::Uploading);
    println!("RESPONSE: check_upload_state {}: {:?}", fake_md5, result);

    println!("WAIT");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("REQUEST: check_upload_state {}", fake_md5);
    let result = check_upload_state(
        recorder.clone(),
        fake_md5.to_string(),
    ).await;
    assert_eq!(result, UploadTaskState::Success);
    println!("RESPONSE: check_upload_state {}: {:?}", fake_md5, result);

    println!("REQUEST: delete_file {}", fake_md5);
    let result = delete_file(
        recorder.clone(),
        fake_md5.to_string(),
    ).await;
    assert!(result);
    println!("RESPONSE: delete_file {}: {:?}", fake_md5, result);

    println!("REQUEST: check_upload_state {}", fake_md5);
    let result = check_upload_state(
        recorder.clone(),
        fake_md5.to_string(),
    ).await;
    assert_eq!(result, UploadTaskState::NotFound);
    println!("RESPONSE: check_upload_state {}: {:?}", fake_md5, result);
}

// APIs -----------

/// launch
async fn upload_file(recorder: AsyncTasksRecorder<Arc<String>>, args: UploadFileArgs) {
    let destination = "some place".to_string(); // decided by some algorithm
    let fut = async move {
        println!("upload_to_destination start!");
        let res = upload_to_destination(args.stream, destination).await;
        match res {
            Ok(msg) => {
                println!("upload_to_destination finish! msg: {}", msg);
                Ok(())
            }
            Err(msg) => {
                println!("upload_to_destination error! msg: {}", msg);
                Err(())
            }
        }
    };

    // launch `Arc<String>` and `Future`
    let _ = recorder.launch(args.md5.into(), fut).await;
}

/// check
async fn check_upload_state(recorder: AsyncTasksRecorder<Arc<String>>, arg_md5: String) -> UploadTaskState {
    let arg_md5 = Arc::new(arg_md5);
    let res = recorder.query_task_state(&arg_md5).await;

    match res {
        TaskState::Success => UploadTaskState::Success,
        TaskState::Failed => UploadTaskState::Failed,
        TaskState::NotFound => UploadTaskState::NotFound,
        TaskState::Working => UploadTaskState::Uploading,
        TaskState::Revoking => UploadTaskState::Revoking,
    }
}

/// revoke
async fn delete_file(recorder: AsyncTasksRecorder<Arc<String>>, arg_md5: String) -> bool {
    let arg_md5 = Arc::new(arg_md5);
    let fut = async move {
        println!("delete_file start!");
        let res = delete().await;
        if res.is_ok() {
            println!("delete_file finish!");
        } else {
            println!("delete_file error!");
        }
        res
    };

    let res = recorder.revoke_task_block(&arg_md5, fut).await;
    res.is_ok()
}

// other functions ------------

async fn upload_to_destination(stream: SimulatedStream, destination: String) -> Result<String, String> {
    // simulate uploading stream to destination
    std::thread::sleep(std::time::Duration::from_millis(50));
    let res = "no problem".to_string(); // result of upload
    // async large callback
    tokio::spawn(large_callback());
    Ok(res)
}

async fn large_callback() {
    println!("large_callback");
    std::thread::sleep(std::time::Duration::from_millis(10));
    println!("large_callback finish");
}

async fn delete() -> Result<(), ()> {
    Ok(())
}
