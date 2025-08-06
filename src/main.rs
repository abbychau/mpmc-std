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

    #[cfg(feature = "simd")]
    mod simd_tests {
        use super::*;
        use mpmc_std::simd_queue::{SimdMpmcQueue, SimdProducer, SimdConsumer};

        #[tokio::test]
        async fn test_simd_basic_batch_operations() {
            let queue = Arc::new(SimdMpmcQueue::<u64>::new(32));
            let producer = SimdProducer::new(Arc::clone(&queue));
            let consumer = SimdConsumer::new(Arc::clone(&queue));

            // Test send
            let send_data = vec![1u64, 2u64, 3u64, 4u64];
            let sent = producer.send(&send_data).unwrap();
            assert_eq!(sent, 4);

            // Test receive
            let mut recv_buffer = vec![0u64; 4];
            let received = consumer.recv(&mut recv_buffer);
            assert_eq!(received, 4);
            assert_eq!(recv_buffer, send_data);
        }

        #[tokio::test]
        async fn test_simd_single_operations() {
            let queue = Arc::new(SimdMpmcQueue::<u64>::new(16));
            let producer = SimdProducer::new(Arc::clone(&queue));
            let consumer = SimdConsumer::new(Arc::clone(&queue));

            // Test single send/recv
            assert!(producer.send_one(42).is_ok());
            let received = consumer.recv_one();
            assert_eq!(received, Some(42));
        }

        #[tokio::test]
        async fn test_simd_mixed_operations() {
            let queue = Arc::new(SimdMpmcQueue::<u64>::new(64));
            let producer = SimdProducer::new(Arc::clone(&queue));
            let consumer = SimdConsumer::new(Arc::clone(&queue));

            // Mix batch and single operations
            let batch = vec![10u64, 20u64, 30u64, 40u64];
            producer.send(&batch).unwrap();
            
            producer.send_one(50).unwrap();
            producer.send_one(60).unwrap();

            // Receive batch first
            let mut recv_buffer = vec![0u64; 4];
            let received = consumer.recv(&mut recv_buffer);
            assert_eq!(received, 4);
            assert_eq!(recv_buffer, batch);

            // Then singles
            assert_eq!(consumer.recv_one(), Some(50));
            assert_eq!(consumer.recv_one(), Some(60));
            assert_eq!(consumer.recv_one(), None);
        }

        #[tokio::test]
        async fn test_simd_concurrent_producers_consumers() {
            let queue = Arc::new(SimdMpmcQueue::<u64>::new(128));
            let mut handles = Vec::new();

            // 2 producers with batch operations
            for producer_id in 0..2 {
                let producer = SimdProducer::new(Arc::clone(&queue));
                handles.push(tokio::spawn(async move {
                    for i in 0..10 {
                        let batch = vec![
                            (producer_id * 40 + i * 4 + 0) as u64,
                            (producer_id * 40 + i * 4 + 1) as u64,
                            (producer_id * 40 + i * 4 + 2) as u64,
                            (producer_id * 40 + i * 4 + 3) as u64,
                        ];
                        while producer.send(&batch).is_err() {
                            tokio::task::yield_now().await;
                        }
                    }
                }));
            }

            // 2 consumers with batch operations
            let received_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            for _ in 0..2 {
                let consumer = SimdConsumer::new(Arc::clone(&queue));
                let counter = Arc::clone(&received_count);
                handles.push(tokio::spawn(async move {
                    let mut buffer = vec![0u64; 4];
                    for _ in 0..10 {
                        loop {
                            let received = consumer.recv(&mut buffer);
                            if received > 0 {
                                counter.fetch_add(received, std::sync::atomic::Ordering::Relaxed);
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

            let total_received = received_count.load(std::sync::atomic::Ordering::Relaxed);
            assert_eq!(total_received, 80); // 2 producers * 10 batches * 4 items
        }

        #[test]
        fn test_simd_queue_properties() {
            let queue = SimdMpmcQueue::<u64>::new(10);
            
            // Capacity should be rounded to next power of 2 and be >= SIMD width * 2
            assert!(queue.capacity() >= 16);
            assert!(queue.capacity().is_power_of_two());
            assert!(queue.is_empty());
            assert!(!queue.is_full());
            assert_eq!(queue.len(), 0);
        }

        #[test]
        fn test_simd_batch_edge_cases() {
            let queue = Arc::new(SimdMpmcQueue::<u64>::new(64)); // Larger capacity
            
            // Test empty batch
            let empty_batch: Vec<u64> = vec![];
            assert_eq!(queue.send(&empty_batch), Ok(0));
            
            // Test partial batches (less than SIMD width)
            let small_batch = vec![1u64, 2u64];
            let sent = queue.send(&small_batch);
            assert_eq!(sent, Ok(2)); // Should send all items
            
            // Test SIMD-aligned batches
            let simd_batch = vec![10u64, 20u64, 30u64, 40u64];
            let sent = queue.send(&simd_batch);
            assert_eq!(sent, Ok(4)); // Should send all items
            
            // Test large batches (more than SIMD width)
            let large_batch = vec![100u64; 7];
            let sent = queue.send(&large_batch);
            assert_eq!(sent, Ok(7)); // Should send all items
            
            // Test receiving data
            let mut recv_buffer = vec![0u64; 15];
            let received = queue.recv(&mut recv_buffer);
            assert_eq!(received, 13); // Should receive all items (2 + 4 + 7)
            
            // Verify we got all the data
            assert_eq!(&recv_buffer[0..2], &[1u64, 2u64]);
            assert_eq!(&recv_buffer[2..6], &[10u64, 20u64, 30u64, 40u64]);
            assert_eq!(&recv_buffer[6..13], &[100u64; 7]);
        }
    }
}