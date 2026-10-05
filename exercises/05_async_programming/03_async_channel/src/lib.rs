use tokio::sync::mpsc;

pub async fn producer_consumer(items: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(items.len().max(1));
    tokio::spawn(async move { for item in items { tx.send(item).await.expect("receiver dropped"); } });
    let mut result = Vec::new();
    while let Some(item) = rx.recv().await { result.push(item); }
    result
}

pub async fn fan_in(n_producers: usize) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(n_producers.max(1));
    let mut handles = Vec::with_capacity(n_producers);
    for id in 0..n_producers {
        let tx = tx.clone();
        handles.push(tokio::spawn(async move { tx.send(format!("producer {id}: message")).await.expect("receiver dropped"); }));
    }
    drop(tx);
    let mut result = Vec::new();
    while let Some(item) = rx.recv().await { result.push(item); }
    for handle in handles { handle.await.expect("producer panicked"); }
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_producer_consumer() {
        let items=vec!["hello".into(),"async".into(),"world".into()];
        assert_eq!(producer_consumer(items.clone()).await,items);
    }
    #[tokio::test]
    async fn test_producer_consumer_empty() { assert!(producer_consumer(Vec::new()).await.is_empty()); }
    #[tokio::test]
    async fn test_fan_in() { assert_eq!(fan_in(3).await, vec!["producer 0: message","producer 1: message","producer 2: message"]); }
    #[tokio::test]
    async fn test_fan_in_single() { assert_eq!(fan_in(1).await, vec!["producer 0: message"]); }
}
