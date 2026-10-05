use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

pub async fn concurrent_squares(n: usize) -> Vec<usize> {
    let handles: Vec<JoinHandle<usize>> = (0..n).map(|i| tokio::spawn(async move { i * i })).collect();
    let mut results = Vec::with_capacity(n);
    for handle in handles { results.push(handle.await.expect("task panicked")); }
    results
}

pub async fn parallel_sleep_tasks(n: usize, duration_ms: u64) -> Vec<usize> {
    let handles: Vec<_> = (0..n).map(|id| tokio::spawn(async move {
        sleep(Duration::from_millis(duration_ms)).await;
        id
    })).collect();
    let mut results = Vec::with_capacity(n);
    for handle in handles { results.push(handle.await.expect("task panicked")); }
    results.sort_unstable();
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;
    #[tokio::test]
    async fn test_squares_basic() { assert_eq!(concurrent_squares(5).await, vec![0,1,4,9,16]); }
    #[tokio::test]
    async fn test_squares_zero() { assert!(concurrent_squares(0).await.is_empty()); }
    #[tokio::test]
    async fn test_squares_one() { assert_eq!(concurrent_squares(1).await, vec![0]); }
    #[tokio::test]
    async fn test_parallel_sleep() {
        let start=Instant::now();
        assert_eq!(parallel_sleep_tasks(5,100).await, vec![0,1,2,3,4]);
        assert!(start.elapsed().as_millis() < 400);
    }
}
