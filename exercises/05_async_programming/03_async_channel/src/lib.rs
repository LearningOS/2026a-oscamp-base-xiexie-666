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
src
lib.rs
Cargo.toml
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
2026a-oscamp-base-xiexie-666/exercises/05_async_programming/03_async_channel/src
/lib.rs
Latest commit
 
History
History
File metadata and controls
Code
Blame
72 lines (64 loc) · 2.34 KB
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
//! # Async Channel
//!
//! In this exercise, you will use `tokio::sync::mpsc` async channels to implement producer-consumer pattern.
//!
//! ## Concepts
//! - `tokio::sync::mpsc::channel` creates bounded async channels
//! - Async `send` and `recv`
//! - Channel closing mechanism (receiver returns None after all senders are dropped)
use tokio::sync::mpsc;


/// Async producer-consumer:
/// - Create a producer task that sends each element from items sequentially
/// - Create a consumer task that receives all elements and collects them into Vec for return
///
/// Hint: Set channel capacity to items.len().max(1)
pub async fn producer_consumer(items: Vec<String>) -> Vec<String> {
    // TODO: Create channel with mpsc::channel
    // TODO: Spawn producer task: iterate through items, send each one
    // TODO: Spawn consumer task: loop recv until channel closes, collect results
    // TODO: Wait for consumer to complete and return results
    let (tx, mut rx) = mpsc::channel(items.len().max(1));
    tokio::spawn(async move { for item in items { tx.send(item).await.unwrap(); } });
    let mut result = Vec::new();
    while let Some(item) = rx.recv().await { result.push(item); }
    result
}


/// Fan‑in pattern: multiple producers, one consumer.
/// Create `n_producers` producers, each sending `"producer {id}: message"`.
/// Consumer collects all messages, sorts them, and returns.
pub async fn fan_in(n_producers: usize) -> Vec<String> {
    // TODO: Create mpsc channel
    // TODO: Spawn n_producers producer tasks
    //       Each sends format!("producer {id}: message")
    // TODO: Drop the original sender (important! otherwise channel won't close)
    // TODO: Consumer loops receiving, collects and sorts
    let (tx, mut rx) = mpsc::channel(n_producers.max(1));
    let mut handles = Vec::with_capacity(n_producers);
    for id in 0..n_producers {
        let tx = tx.clone();
        handles.push(tokio::spawn(async move { tx.send(format!("producer {id}: message")).await.unwrap(); }));
    }
    drop(tx);
    let mut result = Vec::new();
    while let Some(item) = rx.recv().await { result.push(item); }
    for handle in handles { handle.await.unwrap(); }
    result.sort();
    result
}


#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_producer_consumer() {
        let items = vec!["hello".into(), "async".into(), "world".into()];
        let result = producer_consumer(items.clone()).await;
        assert_eq!(result, items);
    }


    #[tokio::test]
    async fn test_producer_consumer_empty() {
        let result = producer_consumer(vec![]).await;
        assert!(result.is_empty());
    }


    #[tokio::test]
    async fn test_fan_in() {
        let result = fan_in(3).await;
        assert_eq!(
            result,
            vec![
                "producer 0: message",
                "producer 1: message",
                "producer 2: message",
            ]
        );
    }


    #[tokio::test]
    async fn test_fan_in_single() {
        let result = fan_in(1).await;
        assert_eq!(result, vec!["producer 0: message"]);
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
 
