use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

#[cfg(feature = "simd")]
use mpmc_std::simd_queue::{SimdMpmcQueue, SimdProducer, SimdConsumer};

use mpmc_std::MpmcQueue;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

#[cfg(feature = "simd")]
fn simd_single_threaded_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_single_threaded_throughput");
    
    for capacity in [64, 256, 1024, 4096].iter() {
        // Compare SIMD batch vs regular send/recv
        group.bench_with_input(BenchmarkId::new("simd_batch", capacity), capacity, |b, &capacity| {
            let queue = Arc::new(SimdMpmcQueue::<u64>::new(capacity));
            let batch_size = 4; // SIMD width for u64x4
            let total_items = 1000;
            let batches = total_items / batch_size;
            
            b.iter(|| {
                let mut send_buffer = vec![42u64; batch_size];
                let mut recv_buffer = vec![0u64; batch_size];
                
                for i in 0..batches {
                    // Fill send buffer with unique values
                    for (j, item) in send_buffer.iter_mut().enumerate() {
                        *item = (i * batch_size + j) as u64;
                    }
                    
                    // Send batch
                    while queue.send_batch(&send_buffer).is_err() {
                        // Queue full, consume some items first
                        queue.recv_batch(&mut recv_buffer);
                    }
                }
                
                // Receive remaining batches
                for _ in 0..batches {
                    while queue.recv_batch(&mut recv_buffer) == 0 {
                        // Queue empty, produce some items first
                        queue.send_batch(&send_buffer).ok();
                    }
                }
                
                black_box(recv_buffer);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("simd_single", capacity), capacity, |b, &capacity| {
            let queue = Arc::new(SimdMpmcQueue::<u64>::new(capacity));
            let total_items = 1000;
            
            b.iter(|| {
                for i in 0..total_items {
                    while queue.send(black_box(i as u64)).is_err() {
                        queue.recv();
                    }
                    if i % 2 == 0 {
                        black_box(queue.recv());
                    }
                }
                
                // Drain remaining items
                while queue.recv().is_some() {}
            });
        });
        
        // Compare with regular queue for the same capacity
        group.bench_with_input(BenchmarkId::new("regular_queue", capacity), capacity, |b, &capacity| {
            let queue = Arc::new(MpmcQueue::<u64>::new(capacity));
            let total_items = 1000;
            
            b.iter(|| {
                for i in 0..total_items {
                    while queue.send(black_box(i as u64)).is_err() {
                        queue.recv();
                    }
                    if i % 2 == 0 {
                        black_box(queue.recv());
                    }
                }
                
                // Drain remaining items
                while queue.recv().is_some() {}
            });
        });
    }
    
    group.finish();
}

#[cfg(feature = "simd")]
fn simd_multi_producer_consumer(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_multi_producer_consumer");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(5));
    
    for thread_pairs in [1, 2, 4, 8].iter() {
        // SIMD batch operations
        group.bench_with_input(
            BenchmarkId::new("simd_batch_mpmc", thread_pairs),
            thread_pairs,
            |b, &thread_pairs| {
                b.iter_custom(|iters| {
                    let queue = Arc::new(SimdMpmcQueue::<u64>::new(1024));
                    let items_per_thread = (iters as usize) / thread_pairs / 4; // Divide by 4 for batch size
                    let batch_size = 4;
                    
                    let start = Instant::now();
                    let mut handles = Vec::new();
                    
                    // Producers with SIMD batches
                    for producer_id in 0..thread_pairs {
                        let queue_clone = Arc::clone(&queue);
                        handles.push(thread::spawn(move || {
                            let mut batch = vec![0u64; batch_size];
                            for i in 0..items_per_thread {
                                // Fill batch
                                for (j, item) in batch.iter_mut().enumerate() {
                                    *item = ((producer_id * items_per_thread + i) * batch_size + j) as u64;
                                }
                                
                                while queue_clone.send_batch(&batch).is_err() {
                                    thread::yield_now();
                                }
                            }
                        }));
                    }
                    
                    // Consumers with SIMD batches
                    for _ in 0..thread_pairs {
                        let queue_clone = Arc::clone(&queue);
                        handles.push(thread::spawn(move || {
                            let mut batch = vec![0u64; batch_size];
                            for _ in 0..items_per_thread {
                                while queue_clone.recv_batch(&mut batch) == 0 {
                                    thread::yield_now();
                                }
                            }
                        }));
                    }
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    
                    start.elapsed()
                });
            }
        );
        
        // Regular single-element operations for comparison
        group.bench_with_input(
            BenchmarkId::new("simd_single_mpmc", thread_pairs),
            thread_pairs,
            |b, &thread_pairs| {
                b.iter_custom(|iters| {
                    let queue = Arc::new(SimdMpmcQueue::<u64>::new(1024));
                    let items_per_thread = (iters as usize) / thread_pairs;
                    
                    let start = Instant::now();
                    let mut handles = Vec::new();
                    
                    // Producers
                    for producer_id in 0..thread_pairs {
                        let queue_clone = Arc::clone(&queue);
                        handles.push(thread::spawn(move || {
                            for i in 0..items_per_thread {
                                let value = (producer_id * items_per_thread + i) as u64;
                                while queue_clone.send(value).is_err() {
                                    thread::yield_now();
                                }
                            }
                        }));
                    }
                    
                    // Consumers
                    for _ in 0..thread_pairs {
                        let queue_clone = Arc::clone(&queue);
                        handles.push(thread::spawn(move || {
                            for _ in 0..items_per_thread {
                                while queue_clone.recv().is_none() {
                                    thread::yield_now();
                                }
                            }
                        }));
                    }
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    
                    start.elapsed()
                });
            }
        );
    }
    
    group.finish();
}

#[cfg(feature = "simd")]
fn simd_batch_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_batch_sizes");
    
    // Test different batch sizes to find optimal SIMD utilization
    for batch_size in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::new("batch_size", batch_size),
            batch_size,
            |b, &batch_size| {
                let queue = Arc::new(SimdMpmcQueue::<u64>::new(1024));
                let total_items = 1000;
                let num_batches = total_items / batch_size;
                
                b.iter(|| {
                    let mut send_batch = vec![42u64; batch_size];
                    let mut recv_batch = vec![0u64; batch_size];
                    
                    // Send all batches
                    for i in 0..num_batches {
                        for (j, item) in send_batch.iter_mut().enumerate() {
                            *item = (i * batch_size + j) as u64;
                        }
                        
                        if batch_size <= 4 {
                            // Use SIMD batch operation for small batches
                            while queue.send_batch(&send_batch).is_err() {
                                thread::yield_now();
                            }
                        } else {
                            // Fall back to individual sends for large batches
                            for &item in &send_batch {
                                while queue.send(item).is_err() {
                                    thread::yield_now();
                                }
                            }
                        }
                    }
                    
                    // Receive all batches
                    for _ in 0..num_batches {
                        if batch_size <= 4 {
                            while queue.recv_batch(&mut recv_batch) == 0 {
                                thread::yield_now();
                            }
                        } else {
                            for item in &mut recv_batch {
                                loop {
                                    match queue.recv() {
                                        Some(value) => {
                                            *item = value;
                                            break;
                                        }
                                        None => thread::yield_now(),
                                    }
                                }
                            }
                        }
                    }
                    
                    black_box(recv_batch);
                });
            }
        );
    }
    
    group.finish();
}

#[cfg(feature = "simd")]
fn simd_latency_measurement(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_latency");
    
    group.bench_function("simd_batch_latency", |b| {
        let queue = Arc::new(SimdMpmcQueue::<u64>::new(1024));
        let batch = vec![42u64; 4];
        let mut recv_buffer = vec![0u64; 4];
        
        b.iter(|| {
            queue.send_batch(&batch).unwrap();
            black_box(queue.recv_batch(&mut recv_buffer));
        });
    });
    
    group.bench_function("simd_single_latency", |b| {
        let queue = Arc::new(SimdMpmcQueue::<u64>::new(1024));
        
        b.iter(|| {
            queue.send(black_box(42u64)).unwrap();
            black_box(queue.recv().unwrap());
        });
    });
    
    group.bench_function("regular_queue_latency", |b| {
        let queue = Arc::new(MpmcQueue::<u64>::new(1024));
        
        b.iter(|| {
            queue.send(black_box(42u64)).unwrap();
            black_box(queue.recv().unwrap());
        });
    });
    
    group.finish();
}

// Fallback benchmarks when SIMD is not enabled
#[cfg(not(feature = "simd"))]
fn simd_disabled_placeholder(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_disabled");
    
    group.bench_function("placeholder", |b| {
        b.iter(|| {
            // Just a placeholder to show SIMD is disabled
            black_box(1 + 1);
        });
    });
    
    group.finish();
}

#[cfg(feature = "simd")]
criterion_group!(
    simd_benches,
    simd_single_threaded_throughput,
    simd_multi_producer_consumer,
    simd_batch_sizes,
    simd_latency_measurement
);

#[cfg(not(feature = "simd"))]
criterion_group!(simd_benches, simd_disabled_placeholder);

criterion_main!(simd_benches);