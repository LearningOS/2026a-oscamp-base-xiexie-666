Skip to content
LearningOS
2026a-oscamp-base-xiexie-666
Type / to search
Repository navigation
Code
Issues
Pull requests
Agents
Actions
Projects
Wiki
Security and quality
Insights
Settings
Files
main
t
T
.cargo
.devcontainer
.github
cli
docs
exercises
01_concurrency_sync
02_no_std_dev
03_os_concurrency
04_context_switch
05_async_programming
01_basic_future
02_tokio_tasks
src
lib.rs
Cargo.toml
03_async_channel
04_select_timeout
06_page_table
scripts
.gitignore
Cargo.lock
Cargo.toml
LICENSE
README.md
check.sh
course.json
enroll.py
exercises.toml
rust-toolchain.toml
students.example.txt
Breadcrumbs
2026a-oscamp-base-xiexie-666/exercises/05_async_programming/02_tokio_tasks/src
/lib.rs
Latest commit
 
History
History
File metadata and controls
Code
Blame
71 lines (62 loc) · 2.29 KB
Raw
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
//! # Tokio Async Tasks
//!
//! In this exercise, you will use `tokio::spawn` to create concurrent asynchronous tasks.
//!
//! ## Concepts
//! - `tokio::spawn` creates asynchronous tasks
//! - `JoinHandle` waits for task completion
//! - Concurrent execution between asynchronous tasks
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};


/// Concurrently compute the square of each number in 0..n, collect results and return in order.
///
/// Hint: Create `tokio::spawn` task for each i, collect JoinHandle, await them sequentially.
pub async fn concurrent_squares(n: usize) -> Vec<usize> {
    // TODO: Create n asynchronous tasks, each computing i * i
    // TODO: Collect all JoinHandle
    // TODO: Await each one to get result
    let handles: Vec<JoinHandle<usize>> = (0..n).map(|i| tokio::spawn(async move { i * i })).collect();
    let mut results = Vec::with_capacity(n);
    for handle in handles { results.push(handle.await.unwrap()); }
    results
}


/// Concurrently execute multiple "time-consuming" tasks (simulated with sleep), return all results.
/// Each task sleeps `duration_ms` milliseconds and then returns its `task_id`.
///
/// Key: All tasks should execute concurrently, total duration should be close to single task duration, not sum of all tasks.
pub async fn parallel_sleep_tasks(n: usize, duration_ms: u64) -> Vec<usize> {
    // TODO: Create asynchronous task for each id in 0..n
    // TODO: Each task sleeps specified duration and returns its own id
    // TODO: Collect all results and sort
    let handles: Vec<_> = (0..n).map(|id| tokio::spawn(async move {
        sleep(Duration::from_millis(duration_ms)).await;
        id
    })).collect();
    let mut results = Vec::with_capacity(n);
    for handle in handles { results.push(handle.await.unwrap()); }
    results.sort_unstable();
    results
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;


    #[tokio::test]
    async fn test_squares_basic() {
        let result = concurrent_squares(5).await;
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }


    #[tokio::test]
    async fn test_squares_zero() {
        let result = concurrent_squares(0).await;
        assert!(result.is_empty());
    }


    #[tokio::test]
    async fn test_squares_one() {
        let result = concurrent_squares(1).await;
        assert_eq!(result, vec![0]);
    }


    #[tokio::test]
    async fn test_parallel_sleep() {
        let start = Instant::now();
        let result = parallel_sleep_tasks(5, 100).await;
        let elapsed = start.elapsed();


        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        // Concurrent execution, total time should be much less than 5 * 100ms
        assert!(
            elapsed.as_millis() < 400,
            "Tasks should run concurrently, took {}ms",
            elapsed.as_millis()
        );
    }
}
Footer
© 2026 GitHub, Inc.
Footer navigation
Terms
Privacy
Security
Status
Community
Docs
Contact
Manage cookies
Do not share my personal information
 
