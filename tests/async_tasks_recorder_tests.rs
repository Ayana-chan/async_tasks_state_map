//! Passed in CPU i7-11800H (8 core 16 thread).

#[allow(unused_imports)]
use async_tasks_state_map::*;
use async_tasks_recorder_tests_repo::*;

mod async_tasks_recorder_tests_repo;

#[test]
fn test_simple_launch_check_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_simple_launch_check(20000),
    );
}

#[test]
fn test_simple_launch_check_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_simple_launch_check(5000),
    );
}

#[test]
fn test_simple_launch_check_revoke_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_simple_launch_check_revoke_loop(20000, 1),
    );
}

#[test]
fn test_simple_launch_check_revoke_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_simple_launch_check_revoke_loop(5000, 1),
    );
}

#[test]
fn test_simple_launch_check_revoke_loop_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_simple_launch_check_revoke_loop(2000, 30),
    );
}

#[test]
fn test_simple_launch_check_revoke_loop_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_simple_launch_check_revoke_loop(1000, 30),
    );
}
