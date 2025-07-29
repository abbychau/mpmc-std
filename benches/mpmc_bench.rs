use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use mpmc_std::MpmcQueue;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn single_threaded_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_threaded_throughput");
    
    for capacity in [64, 256, 1024, 4096].iter() {
        group.bench_with_input(BenchmarkId::new("send_recv", capacity), capacity, |b, &capacity| {
            let queue = Arc::new(MpmcQueue::new(capacity));
            let batch_size = std::cmp::min(capacity / 2, 1000);
            
            b.iter(|| {
                // Send items
                for i in 0..batch_size {
                    while queue.send(black_box(i)).is_err() {
                        // Queue full, consume some items first
                        if queue.recv().is_some() {
                            break;
                        }
                    }
                }
                // Receive items
                for _ in 0..batch_size {
                    while queue.recv().is_none() {
                        // Queue empty, produce some items first
                        if queue.send(black_box(999)).is_ok() {
                            break;
                        }
                    }
                }
            });
        });
    }
    
    group.finish();
}

fn multi_producer_single_consumer(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_producer_single_consumer");
    
    for num_producers in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("producers", num_producers), 
            num_producers, 
            |b, &num_producers| {
                b.iter_custom(|iters| {
                    let queue = Arc::new(MpmcQueue::new(1024));
                    let items_per_producer = (iters as usize) / num_producers;
                    
                    let start = Instant::now();
                    
                    let mut producer_handles = Vec::new();
                    
                    // Spawn producers
                    for producer_id in 0..num_producers {
                        let queue_clone = Arc::clone(&queue);
                        let handle = thread::spawn(move || {
                            for i in 0..items_per_producer {
                                let value = producer_id * items_per_producer + i;
                                while queue_clone.send(black_box(value)).is_err() {
                                    thread::yield_now();
                                }
                            }
                        });
                        producer_handles.push(handle);
                    }
                    
                    // Consumer
                    let consumer_queue = Arc::clone(&queue);
                    let consumer_handle = thread::spawn(move || {
                        let total_items = items_per_producer * num_producers;
                        for _ in 0..total_items {
                            while consumer_queue.recv().is_none() {
                                thread::yield_now();
                            }
                        }
                    });
                    
                    // Wait for completion
                    for handle in producer_handles {
                        handle.join().unwrap();
                    }
                    consumer_handle.join().unwrap();
                    
                    start.elapsed()
                });
            }
        );
    }
    
    group.finish();
}

fn single_producer_multi_consumer(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_producer_multi_consumer");
    
    for num_consumers in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("consumers", num_consumers), 
            num_consumers, 
            |b, &num_consumers| {
                b.iter_custom(|iters| {
                    let queue = Arc::new(MpmcQueue::new(1024));
                    let total_items = iters as usize;
                    
                    let start = Instant::now();
                    
                    // Producer
                    let producer_queue = Arc::clone(&queue);
                    let producer_handle = thread::spawn(move || {
                        for i in 0..total_items {
                            while producer_queue.send(black_box(i)).is_err() {
                                thread::yield_now();
                            }
                        }
                    });
                    
                    // Spawn consumers
                    let mut consumer_handles = Vec::new();
                    let items_per_consumer = Arc::new(std::sync::atomic::AtomicUsize::new(0));
                    
                    for _ in 0..num_consumers {
                        let queue_clone = Arc::clone(&queue);
                        let counter_clone = Arc::clone(&items_per_consumer);
                        let handle = thread::spawn(move || {
                            let mut count = 0;
                            while count < total_items / num_consumers + 1 {
                                if queue_clone.recv().is_some() {
                                    count += 1;
                                    counter_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                } else {
                                    thread::yield_now();
                                }
                                
                                if counter_clone.load(std::sync::atomic::Ordering::Relaxed) >= total_items {
                                    break;
                                }
                            }
                        });
                        consumer_handles.push(handle);
                    }
                    
                    // Wait for completion
                    producer_handle.join().unwrap();
                    for handle in consumer_handles {
                        handle.join().unwrap();
                    }
                    
                    start.elapsed()
                });
            }
        );
    }
    
    group.finish();
}

fn multi_producer_multi_consumer(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_producer_multi_consumer");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(5));
    
    // Simple approach: measure average time per operation in MPMC scenario
    for thread_pairs in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("thread_pairs", thread_pairs), 
            thread_pairs, 
            |b, &thread_pairs| {
                let queue = Arc::new(MpmcQueue::new(1024)); // Larger queue to avoid contention
                
                b.iter(|| {
                    // Pre-fill queue with some items
                    for i in 0..10 {
                        let _ = queue.send(black_box(i));
                    }
                    
                    // Simple concurrent access pattern
                    let barrier = Arc::new(std::sync::Barrier::new(thread_pairs * 2));
                    let mut handles = Vec::new();
                    
                    // Producers
                    for _ in 0..thread_pairs {
                        let queue_clone = Arc::clone(&queue);
                        let barrier_clone = Arc::clone(&barrier);
                        handles.push(std::thread::spawn(move || {
                            barrier_clone.wait();
                            for i in 0..10 {
                                while queue_clone.send(black_box(i)).is_err() {
                                    std::hint::spin_loop();
                                }
                            }
                        }));
                    }
                    
                    // Consumers
                    for _ in 0..thread_pairs {
                        let queue_clone = Arc::clone(&queue);
                        let barrier_clone = Arc::clone(&barrier);
                        handles.push(std::thread::spawn(move || {
                            barrier_clone.wait();
                            for _ in 0..10 {
                                while queue_clone.recv().is_none() {
                                    std::hint::spin_loop();
                                }
                            }
                        }));
                    }
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    
                    // Drain remaining items
                    while queue.recv().is_some() {}
                });
            }
        );
    }
    
    group.finish();
}

fn latency_measurement(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency");
    
    group.bench_function("send_latency", |b| {
        let queue = Arc::new(MpmcQueue::new(1024));
        
        b.iter(|| {
            queue.send(black_box(42)).unwrap();
            black_box(queue.recv().unwrap());
        });
    });
    
    group.bench_function("recv_latency", |b| {
        let queue = Arc::new(MpmcQueue::new(1024));
        // Pre-fill queue
        for i in 0..1000 {
            queue.send(i).unwrap();
        }
        
        b.iter(|| {
            black_box(queue.recv().unwrap());
            queue.send(black_box(999)).unwrap();
        });
    });
    
    group.finish();
}

fn contention_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("contention");
    
    for queue_size in [16, 64, 256].iter() {
        group.bench_with_input(
            BenchmarkId::new("high_contention", queue_size),
            queue_size,
            |b, &queue_size| {
                b.iter_custom(|iters| {
                    let queue = Arc::new(MpmcQueue::new(queue_size));
                    let num_threads = 16;
                    let items_per_thread = (iters as usize) / num_threads;
                    
                    let start = Instant::now();
                    let mut handles = Vec::new();
                    
                    // Half producers, half consumers
                    for i in 0..num_threads {
                        let queue_clone = Arc::clone(&queue);
                        
                        if i < num_threads / 2 {
                            // Producer
                            let handle = thread::spawn(move || {
                                for j in 0..items_per_thread {
                                    let value = i * items_per_thread + j;
                                    while queue_clone.send(black_box(value)).is_err() {
                                        thread::yield_now();
                                    }
                                }
                            });
                            handles.push(handle);
                        } else {
                            // Consumer
                            let handle = thread::spawn(move || {
                                for _ in 0..items_per_thread {
                                    while queue_clone.recv().is_none() {
                                        thread::yield_now();
                                    }
                                }
                            });
                            handles.push(handle);
                        }
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

criterion_group!(
    benches,
    single_threaded_throughput,
    multi_producer_single_consumer,
    single_producer_multi_consumer,
    multi_producer_multi_consumer,
    latency_measurement,
    contention_benchmark
);
criterion_main!(benches);