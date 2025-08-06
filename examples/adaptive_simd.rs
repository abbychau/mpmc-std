use std::sync::Arc;

#[cfg(feature = "simd")]
use mpmc_std::simd_queue::{SimdMpmcQueue, SimdProducer, SimdConsumer};

fn main() {
    #[cfg(feature = "simd")]
    {
        println!("🔬 SIMD Adaptive Queue Demo");
        println!("============================");
        
        let queue = Arc::new(SimdMpmcQueue::<u64>::new(2048));
        let producer = SimdProducer::new(Arc::clone(&queue));
        let consumer = SimdConsumer::new(Arc::clone(&queue));
        
        // Test Case 1: Send 7 items adaptively (uses SIMD for first 4, then 3 singles)
        println!("\n1. Send: 7 items");
        let data = vec![10u64, 20u64, 30u64, 40u64, 50u64, 60u64, 70u64];
        match producer.send(&data) {
            Ok(sent) => println!("   ✅ Sent {} items adaptively", sent),
            Err(unsent) => println!("   ⚠️  Sent {} items, {} remaining", data.len() - unsent.len(), unsent.len()),
        }
        
        println!("   Queue length after send: {}", queue.len());
        
        // Test Case 2: Receive into 10-item buffer (gets all 7 items)  
        println!("\n2. Receive: 10-item buffer");
        let mut buffer = vec![0u64; 10];
        let received = consumer.recv(&mut buffer);
        println!("   ✅ Received {} items adaptively", received);
        println!("   Data: {:?}", &buffer[..received]);
        println!("   Queue length after receive: {}", queue.len());
        
        // Test Case 3: Demonstrate partial batch behavior
        println!("\n3. Partial Batch Demo");
        
        // Send 3 items (not enough for SIMD batch)
        let partial_data = vec![100u64, 200u64, 300u64];
        producer.send(&partial_data).unwrap();
        println!("   📤 Sent 3 items (processed as singles)");
        
        // Try to receive with 4-item buffer (should get all 3)
        let mut small_buffer = vec![0u64; 4];
        let partial_received = consumer.recv(&mut small_buffer);
        println!("   📥 Received {} items into 4-item buffer", partial_received);
        println!("   Data: {:?}", &small_buffer[..partial_received]);
        
        // Test Case 4: Compare with batch operations
        println!("\n4. Comparison with Legacy Batch Operations");
        
        // Fill queue with 5 items
        let test_data = vec![1u64, 2u64, 3u64, 4u64, 5u64];
        producer.send(&test_data).unwrap();
        
        // Receive first batch of 4 items
        let mut batch_buffer = vec![0u64; 4];
        let batch_received = consumer.recv(&mut batch_buffer);
        println!("   🔄 recv(4): got {} items", batch_received);
        println!("   Data: {:?}", &batch_buffer[..batch_received]);
        println!("   Remaining in queue: {}", queue.len());
        
        // Receive the remaining item
        let mut adaptive_buffer = vec![0u64; 4];
        let adaptive_received = consumer.recv(&mut adaptive_buffer);
        println!("   🚀 recv: got {} more items", adaptive_received);
        println!("   Data: {:?}", &adaptive_buffer[..adaptive_received]);
        
        // Test Case 5: Performance comparison
        println!("\n5. Performance Comparison");
        
        // Fill queue with lots of data
        let large_data: Vec<u64> = (0..1000u64).collect();
        let start = std::time::Instant::now();
        producer.send(&large_data).unwrap();
        let adaptive_send_time = start.elapsed();
        
        // Receive all data
        let mut receive_buffer = vec![0u64; 1000];
        let start = std::time::Instant::now();
        let total_received = consumer.recv(&mut receive_buffer);
        let adaptive_recv_time = start.elapsed();
        
        println!("   📊 Sent 1000 items in {:?} ({:.0} ops/sec)", 
                 adaptive_send_time, 
                 1000.0 / adaptive_send_time.as_secs_f64());
        println!("   📊 Received {} items in {:?} ({:.0} ops/sec)", 
                 total_received,
                 adaptive_recv_time, 
                 total_received as f64 / adaptive_recv_time.as_secs_f64());
        
        println!("\n🎯 Summary:");
        println!("   • send/recv methods automatically use SIMD for groups of 4");
        println!("   • Remaining items (1-3) processed individually");  
        println!("   • No data loss - all items are processed");
        println!("   • Optimal performance without manual batch management");
    }
    
    #[cfg(not(feature = "simd"))]
    {
        println!("❌ SIMD features not enabled!");
        println!("Run with: cargo run --features simd --example adaptive_simd");
    }
}