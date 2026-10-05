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
03_async_channel
04_select_timeout
src
lib.rs
Cargo.toml
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
2026a-oscamp-base-xiexie-666/exercises/05_async_programming/04_select_timeout/src
/lib.rs
Latest commit
 
History
History
File metadata and controls
Code
Blame
94 lines (86 loc) · 2.39 KB
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
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
//! # Select and Timeout
//!
//! In this exercise, you will use `tokio::select!` macro to implement race selection and timeout control.
//!
//! ## Concepts
//! - `tokio::select!` waits for multiple async operations simultaneously
//! - `tokio::time::timeout` timeout control
//! - The first completed branch is executed, others are cancelled
use std::future::Future;
use tokio::time::{sleep, Duration};


/// Async operation with timeout.
/// If `future` completes within `timeout_ms` milliseconds, returns Some(result).
/// Otherwise returns None.
///
/// Hint: Use `tokio::select!` or `tokio::time::timeout`.
pub async fn with_timeout<F, T>(future: F, timeout_ms: u64) -> Option<T>
where
    F: Future<Output = T>,
{
    // TODO: Use tokio::select! to race between future and sleep
    // Or use tokio::time::timeout
    tokio::time::timeout(Duration::from_millis(timeout_ms), future).await.ok()
}


/// Race two async tasks, return the result of whichever finishes first.
///
/// Hint: Use `tokio::select!` macro.
pub async fn race<F1, F2, T>(f1: F1, f2: F2) -> T
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    // TODO: Use tokio::select! to wait for f1 and f2
    // Return the result of whichever completes first
    tokio::pin!(f1);
    tokio::pin!(f2);
    tokio::select! { value = &mut f1 => value, value = &mut f2 => value }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_timeout_success() {
        let result = with_timeout(async { 42 }, 100).await;
        assert_eq!(result, Some(42));
    }


    #[tokio::test]
    async fn test_timeout_expired() {
        let result = with_timeout(
            async {
                sleep(Duration::from_millis(200)).await;
                42
            },
            50,
        )
        .await;
        assert_eq!(result, None);
    }


    #[tokio::test]
    async fn test_race_first_wins() {
        let result = race(
            async {
                sleep(Duration::from_millis(10)).await;
                "fast"
            },
            async {
                sleep(Duration::from_millis(200)).await;
                "slow"
            },
        )
        .await;
        assert_eq!(result, "fast");
    }


    #[tokio::test]
    async fn test_race_second_wins() {
        let result = race(
            async {
                sleep(Duration::from_millis(200)).await;
                "slow"
            },
            async {
                sleep(Duration::from_millis(10)).await;
                "fast"
            },
        )
        .await;
        assert_eq!(result, "fast");
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
 
