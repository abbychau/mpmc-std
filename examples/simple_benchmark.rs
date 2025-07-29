use mpmc_std::MpmcQueue;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    println!("MPMC Queue Performance Benchmark");
    println!("================================");
    
    // Single-threaded throughput test
    println!("\n1. Single-threaded throughput test:");
    for capacity in [64, 256, 1024] {
        let queue = Arc::new(MpmcQueue::new(capacity));
        let start = Instant::now();
        let iterations = 100_000;
        
        for i in 0..iterations {
            while queue.send(i).is_err() {
                queue.recv();
            }
            if i % 2 == 0 {
                queue.recv();
            }
        }
        
        // Drain remaining items
        while queue.recv().is_some() {}
        
        let duration = start.elapsed();
        let ops_per_sec = (iterations * 2) as f64 / duration.as_secs_f64();
        println!("  Capacity {}: {:.0} ops/sec", capacity, ops_per_sec);
    }
    
    // Multi-threaded contention test
    println!("\n2. Multi-threaded contention test (4 producers, 4 consumers):");
    for capacity in [64, 256, 1024] {
        let queue = Arc::new(MpmcQueue::new(capacity));
        let items_per_thread = 25_000;
        let start = Instant::now();
        
        let mut handles = Vec::new();
        
        // Spawn producers
        for i in 0..4 {
            let queue_clone = Arc::clone(&queue);
            let handle = thread::spawn(move || {
                for j in 0..items_per_thread {
                    let value = i * items_per_thread + j;
                    while queue_clone.send(value).is_err() {
                        thread::yield_now();
                    }
                }
            });
            handles.push(handle);
        }
        
        // Spawn consumers
        for _ in 0..4 {
            let queue_clone = Arc::clone(&queue);
            let handle = thread::spawn(move || {
                for _ in 0..items_per_thread {
                    while queue_clone.recv().is_none() {
                        thread::yield_now();
                    }
                }
            });
            handles.push(handle);
        }
        
        // Wait for completion
        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = start.elapsed();
        let total_ops = items_per_thread * 8; // 4 producers + 4 consumers
        let ops_per_sec = total_ops as f64 / duration.as_secs_f64();
        println!("  Capacity {}: {:.0} ops/sec", capacity, ops_per_sec);
    }
    
    // Latency test
    println!("\n3. Latency test (send-receive pairs):");
    let queue = Arc::new(MpmcQueue::new(1024));
    let iterations = 100_000;
    let start = Instant::now();
    
    for i in 0..iterations {
        queue.send(i).unwrap();
        queue.recv().unwrap();
    }
    
    let duration = start.elapsed();
    let avg_latency = duration.as_nanos() / (iterations * 2) as u128;
    println!("  Average operation latency: {} ns", avg_latency);
    
    // High contention test
    println!("\n4. High contention test (16 threads, small queue):");
    let queue = Arc::new(MpmcQueue::new(32));
    let items_per_thread = 10_000;
    let start = Instant::now();
    
    let mut handles = Vec::new();
    
    // 8 producers, 8 consumers
    for i in 0..8 {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for j in 0..items_per_thread {
                let value = i * items_per_thread + j;
                while queue_clone.send(value).is_err() {
                    thread::yield_now();
                }
            }
        });
        handles.push(handle);
    }
    
    for _ in 0..8 {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for _ in 0..items_per_thread {
                while queue_clone.recv().is_none() {
                    thread::yield_now();
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    let total_ops = items_per_thread * 16;
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();
    println!("  High contention: {:.0} ops/sec", ops_per_sec);
    
    println!("\nBenchmark completed!");
}