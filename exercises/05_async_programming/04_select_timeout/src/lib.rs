use std::future::Future;
use tokio::time::{sleep, Duration};

pub async fn with_timeout<F, T>(future: F, timeout_ms: u64) -> Option<T>
where F: Future<Output = T> {
    tokio::time::timeout(Duration::from_millis(timeout_ms), future).await.ok()
}

pub async fn race<F1, F2, T>(f1: F1, f2: F2) -> T
where F1: Future<Output = T>, F2: Future<Output = T> {
    tokio::pin!(f1);
    tokio::pin!(f2);
    tokio::select! { value = &mut f1 => value, value = &mut f2 => value }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_timeout_success() { assert_eq!(with_timeout(async {42},100).await,Some(42)); }
    #[tokio::test]
    async fn test_timeout_expired() { assert_eq!(with_timeout(async { sleep(Duration::from_millis(200)).await; 42 },50).await,None); }
    #[tokio::test]
    async fn test_race_first_wins() { assert_eq!(race(async {sleep(Duration::from_millis(10)).await; "fast"},async {sleep(Duration::from_millis(200)).await; "slow"}).await,"fast"); }
    #[tokio::test]
    async fn test_race_second_wins() { assert_eq!(race(async {sleep(Duration::from_millis(200)).await; "slow"},async {sleep(Duration::from_millis(10)).await; "fast"}).await,"fast"); }
}
