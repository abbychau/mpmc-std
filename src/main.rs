use mpmc_std::{MpmcQueue, Producer, Consumer};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("High-Performance MPMC Queue Demo");
    println!("================================");
    
    let queue = Arc::new(MpmcQueue::new(64)); // Will be rounded to next power of 2
    
    let producer1 = Producer::new(Arc::clone(&queue));
    let producer2 = Producer::new(Arc::clone(&queue));
    let consumer1 = Consumer::new(Arc::clone(&queue));
    let consumer2 = Consumer::new(Arc::clone(&queue));
    
    println!("Queue capacity: {}", queue.capacity());
    
    // Spawn async tasks using the new synchronous API
    tokio::spawn(async move {
        for i in 0..10 {
            match producer1.send(format!("Producer1: {}", i)) {
                Ok(()) => println!("✓ Producer1 sent: {}", i),
                Err(item) => println!("✗ Producer1 failed to send: {}", item),
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });
    
    tokio::spawn(async move {
        for i in 0..10 {
            match producer2.send(format!("Producer2: {}", i)) {
                Ok(()) => println!("✓ Producer2 sent: {}", i),
                Err(item) => println!("✗ Producer2 failed to send: {}", item),
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(75)).await;
        }
    });
    
    tokio::spawn(async move {
        for _ in 0..10 {
            match consumer1.recv() {
                Some(msg) => println!("📨 Consumer1 received: {}", msg),
                None => println!("📭 Consumer1 found empty queue"),
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });
    
    tokio::spawn(async move {
        for _ in 0..10 {
            match consumer2.recv() {
                Some(msg) => println!("📨 Consumer2 received: {}", msg),
                None => println!("📭 Consumer2 found empty queue"),
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(120)).await;
        }
    });
    
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    println!("\nFinal queue state:");
    println!("- Capacity: {}", queue.capacity());
    println!("- Length: {}", queue.len());
    println!("- Is empty: {}", queue.is_empty());
    println!("- Is full: {}", queue.is_full());
    
    println!("\nMPMC Queue demo completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_send_recv() {
        let queue = Arc::new(MpmcQueue::new(5));
        let producer = Producer::new(Arc::clone(&queue));
        let consumer = Consumer::new(Arc::clone(&queue));

        assert!(producer.send(42).is_ok());
        let received = consumer.recv();
        assert_eq!(received, Some(42));
    }

    #[tokio::test]
    async fn test_multiple_producers_consumers() {
        let queue = Arc::new(MpmcQueue::new(16));
        let mut producer_handles = Vec::new();
        let mut consumer_handles = Vec::new();

        // Spawn 3 producers
        for i in 0..3 {
            let producer = Producer::new(Arc::clone(&queue));
            producer_handles.push(tokio::spawn(async move {
                for j in 0..5 {
                    let value = i * 100 + j;
                    while producer.send(value).is_err() {
                        tokio::task::yield_now().await;
                    }
                }
            }));
        }

        // Spawn 3 consumers
        for _ in 0..3 {
            let consumer = Consumer::new(Arc::clone(&queue));
            let handle = tokio::spawn(async move {
                let mut items = Vec::new();
                for _ in 0..5 {
                    loop {
                        if let Some(item) = consumer.recv() {
                            items.push(item);
                            break;
                        }
                        tokio::task::yield_now().await;
                    }
                }
                items
            });
            consumer_handles.push(handle);
        }

        // Wait for producers to finish
        for handle in producer_handles {
            handle.await.unwrap();
        }

        // Collect results from consumers
        let mut received = Vec::new();
        for handle in consumer_handles {
            if let Ok(items) = handle.await {
                received.extend(items);
            }
        }

        received.sort();
        let mut expected: Vec<i32> = (0..3).flat_map(|i| (0..5).map(move |j| i * 100 + j)).collect();
        expected.sort();
        assert_eq!(received, expected);
    }

    #[tokio::test]
    async fn test_bounded_capacity() {
        let queue = Arc::new(MpmcQueue::new(2));
        let producer = Producer::new(Arc::clone(&queue));

        assert!(producer.send(1).is_ok());
        assert!(producer.send(2).is_ok());

        // Queue should be full now
        let send_result = producer.send(3);
        assert!(send_result.is_err());
        assert_eq!(send_result.unwrap_err(), 3);
    }

    #[tokio::test]
    async fn test_empty_queue_recv() {
        let queue: Arc<MpmcQueue<i32>> = Arc::new(MpmcQueue::new(5));
        let consumer = Consumer::new(Arc::clone(&queue));

        let recv_result = consumer.recv();
        assert_eq!(recv_result, None);
    }

    #[test]
    fn test_queue_properties() {
        let queue = MpmcQueue::<i32>::new(10);
        
        // Capacity should be rounded to next power of 2
        assert_eq!(queue.capacity(), 16);
        assert!(queue.is_empty());
        assert!(!queue.is_full());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_synchronous_operations() {
        let queue = Arc::new(MpmcQueue::new(4));
        
        // Test synchronous send/recv
        assert!(queue.send(1).is_ok());
        assert!(queue.send(2).is_ok());
        assert_eq!(queue.len(), 2);
        
        assert_eq!(queue.recv(), Some(1));
        assert_eq!(queue.recv(), Some(2));
        assert_eq!(queue.recv(), None);
        assert!(queue.is_empty());
    }

    #[tokio::test]
    async fn test_high_contention() {
        let queue = Arc::new(MpmcQueue::new(32));
        let mut handles = Vec::new();

        // 8 producers, 8 consumers
        for i in 0..8 {
            let producer = Producer::new(Arc::clone(&queue));
            handles.push(tokio::spawn(async move {
                for j in 0..50 {
                    let value = i * 50 + j;
                    while producer.send(value).is_err() {
                        tokio::task::yield_now().await;
                    }
                }
            }));
        }

        let consumed_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        for _ in 0..8 {
            let consumer = Consumer::new(Arc::clone(&queue));
            let counter = Arc::clone(&consumed_count);
            handles.push(tokio::spawn(async move {
                for _ in 0..50 {
                    loop {
                        if consumer.recv().is_some() {
                            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            break;
                        }
                        tokio::task::yield_now().await;
                    }
                }
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let total_consumed = consumed_count.load(std::sync::atomic::Ordering::Relaxed);

        assert_eq!(total_consumed, 400); // 8 producers * 50 items each
    }
}