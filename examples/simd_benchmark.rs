use std::sync::Arc;
use std::thread;
use std::time::Instant;

#[cfg(feature = "simd")]
use mpmc_std::simd_queue::{SimdMpmcQueue, SimdProducer, SimdConsumer};

use mpmc_std::MpmcQueue;

fn main() {
    println!("🚀 SIMD MPMC Queue Performance Comparison");
    println!("==========================================");
    
    #[cfg(feature = "simd")]
    {
        println!("\n📊 SIMD Support: ENABLED");
        run_simd_benchmarks();
    }
    
    #[cfg(not(feature = "simd"))]
    {
        println!("\n📊 SIMD Support: DISABLED");
        println!("To enable SIMD optimizations, compile with: cargo run --features simd --example simd_benchmark");
        run_regular_benchmarks_only();
    }
}

#[cfg(feature = "simd")]
fn run_simd_benchmarks() {
    // 1. Single-threaded throughput comparison
    println!("\n1. Single-threaded Operations:");
    for capacity in [64, 256, 1024] {
        let iterations = 10_000;
        
        // SIMD queue operations
        let simd_queue = Arc::new(SimdMpmcQueue::<u64>::new(capacity));
        let producer = SimdProducer::new(Arc::clone(&simd_queue));
        let consumer = SimdConsumer::new(Arc::clone(&simd_queue));
        
        let start = Instant::now();
        
        // Test with different batch sizes
        let batch_data = vec![1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64];
        for _ in 0..iterations / batch_data.len() {
            producer.send(&batch_data).unwrap();
            
            let mut recv_buffer = vec![0u64; batch_data.len()];
            consumer.recv(&mut recv_buffer);
        }
        
        let simd_duration = start.elapsed();
        let simd_ops_per_sec = (iterations * 2) as f64 / simd_duration.as_secs_f64();
        
        // Single operations with SIMD queue
        let start = Instant::now();
        for i in 0..iterations {
            while producer.send_one(i as u64).is_err() {
                // Drain some items to make space
                consumer.recv_one();
            }
            if i % 2 == 0 {
                consumer.recv_one();
            }
        }
        // Drain remaining
        while consumer.recv_one().is_some() {}
        
        let single_duration = start.elapsed();
        let single_ops_per_sec = (iterations * 2) as f64 / single_duration.as_secs_f64();
        
        // Regular queue for comparison
        let regular_queue = Arc::new(MpmcQueue::<u64>::new(capacity));
        let start = Instant::now();
        for i in 0..iterations {
            while regular_queue.send(i as u64).is_err() {
                // Drain some items to make space
                regular_queue.recv();
            }
            if i % 2 == 0 {
                regular_queue.recv();
            }
        }
        while regular_queue.recv().is_some() {}
        
        let regular_duration = start.elapsed();
        let regular_ops_per_sec = (iterations * 2) as f64 / regular_duration.as_secs_f64();
        
        println!("   Capacity {}: SIMD batch: {:.0} ops/sec, SIMD single: {:.0} ops/sec, Regular: {:.0} ops/sec",
                capacity, simd_ops_per_sec, single_ops_per_sec, regular_ops_per_sec);
    }
    
    // 2. Multi-threaded comparison
    println!("\n2. Multi-threaded Performance (4 producer/consumer pairs):");
    for capacity in [64, 256, 1024] {
        let iterations_per_thread = 10_000;
        
        // SIMD multi-threaded test
        let simd_queue = Arc::new(SimdMpmcQueue::<u64>::new(capacity));
        let start = Instant::now();
        
        let mut handles = Vec::new();
        
        // Spawn 4 producer-consumer pairs
        for thread_id in 0..4 {
            let producer = SimdProducer::new(Arc::clone(&simd_queue));
            let consumer = SimdConsumer::new(Arc::clone(&simd_queue));
            
            let handle = thread::spawn(move || {
                // Producer
                let data_batch = vec![thread_id as u64; 7];
                for _ in 0..iterations_per_thread / 7 {
                    while producer.send(&data_batch).is_err() {
                        thread::yield_now();
                    }
                }
                
                // Consumer
                let mut buffer = vec![0u64; 10];
                for _ in 0..iterations_per_thread / 10 {
                    while consumer.recv(&mut buffer) == 0 {
                        thread::yield_now();
                    }
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let simd_mt_duration = start.elapsed();
        let simd_mt_ops_per_sec = (iterations_per_thread * 4 * 2) as f64 / simd_mt_duration.as_secs_f64();
        
        // Regular queue multi-threaded test
        let regular_queue = Arc::new(MpmcQueue::<u64>::new(capacity));
        let start = Instant::now();
        
        let mut handles = Vec::new();
        
        for thread_id in 0..4 {
            let queue = Arc::clone(&regular_queue);
            let handle = thread::spawn(move || {
                for i in 0..iterations_per_thread {
                    while queue.send((thread_id * 1000 + i) as u64).is_err() {
                        thread::yield_now();
                    }
                }
                
                for _ in 0..iterations_per_thread {
                    while queue.recv().is_none() {
                        thread::yield_now();
                    }
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let regular_mt_duration = start.elapsed();
        let regular_mt_ops_per_sec = (iterations_per_thread * 4 * 2) as f64 / regular_mt_duration.as_secs_f64();
        
        let improvement = (simd_mt_ops_per_sec / regular_mt_ops_per_sec - 1.0) * 100.0;
        
        println!("   Capacity {}: SIMD: {:.0} ops/sec, Regular: {:.0} ops/sec ({:+.1}%)",
                capacity, simd_mt_ops_per_sec, regular_mt_ops_per_sec, improvement);
    }
    
    // 3. Different data types test
    println!("\n3. Different 64-bit Data Types:");
    
    // Test with i64
    let i64_queue = Arc::new(SimdMpmcQueue::<i64>::new(64));
    let i64_producer = SimdProducer::new(Arc::clone(&i64_queue));
    let i64_consumer = SimdConsumer::new(Arc::clone(&i64_queue));
    
    let i64_data = vec![-1i64, -2i64, -3i64, -4i64];
    i64_producer.send(&i64_data).unwrap();
    let mut i64_buffer = vec![0i64; 4];
    let received = i64_consumer.recv(&mut i64_buffer);
    println!("   i64 test: sent {:?}, received {} items: {:?}", i64_data, received, &i64_buffer[..received]);
    
    // Test with f64
    let f64_queue = Arc::new(SimdMpmcQueue::<f64>::new(64));
    let f64_producer = SimdProducer::new(Arc::clone(&f64_queue));
    let f64_consumer = SimdConsumer::new(Arc::clone(&f64_queue));
    
    let f64_data = vec![1.1f64, 2.2f64, 3.3f64, 4.4f64, 5.5f64];
    f64_producer.send(&f64_data).unwrap();
    let mut f64_buffer = vec![0.0f64; 5];
    let received = f64_consumer.recv(&mut f64_buffer);
    println!("   f64 test: sent {:?}, received {} items: {:?}", f64_data, received, &f64_buffer[..received]);
    
    println!("\n🎯 SIMD optimization automatically uses vectorized operations for groups of 4 elements");
    println!("   and falls back to single operations for remaining items (1-3).");
}

#[cfg(not(feature = "simd"))]
fn run_regular_benchmarks_only() {
    println!("\nRunning regular queue benchmarks only...");
    
    for capacity in [64, 256, 1024] {
        let iterations = 100_000;
        let queue = Arc::new(MpmcQueue::<u64>::new(capacity));
        
        let start = Instant::now();
        for i in 0..iterations {
            queue.send(i as u64).unwrap();
            if i % 2 == 0 {
                queue.recv();
            }
        }
        while queue.recv().is_some() {}
        
        let duration = start.elapsed();
        let ops_per_sec = (iterations * 2) as f64 / duration.as_secs_f64();
        
        println!("   Capacity {}: {:.0} ops/sec", capacity, ops_per_sec);
    }
}