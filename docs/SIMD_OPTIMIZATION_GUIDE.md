# SIMD Optimization Guide

This guide covers the SIMD-optimized MPMC queue implementation, providing detailed usage instructions, performance analysis, and best practices for achieving optimal throughput.

## Overview

The SIMD-optimized MPMC queue (`SimdMpmcQueue<u64>`) uses vectorized instructions to process multiple elements simultaneously, providing significant performance improvements for u64 numeric workloads.

**🚀 New: Adaptive Operations** - The queue now provides adaptive `send_adaptive()` and `recv_adaptive()` methods that automatically choose between SIMD batch operations and single-element operations based on available data, eliminating the need for manual batch management.

## Quick Start

### Prerequisites

**Rust Toolchain:**
```bash
rustup toolchain install nightly
rustup default nightly
```

**Project Configuration:**
```toml
# Cargo.toml
[features]
simd = []
default = ["simd"]

[dependencies]
mpmc-std = { version = "0.1.0", features = ["simd"] }
```

### Basic Usage

```rust
use mpmc_std::simd_queue::{SimdMpmcQueue, SimdProducer, SimdConsumer};
use std::sync::Arc;

fn main() {
    // Create SIMD-optimized queue for u64 data
    let queue = Arc::new(SimdMpmcQueue::<u64>::new(64));
    let producer = SimdProducer::new(Arc::clone(&queue));
    let consumer = SimdConsumer::new(Arc::clone(&queue));
    
    // Adaptive operations (recommended) - automatically uses SIMD when beneficial
    let data = vec![100u64, 200u64, 300u64, 400u64, 500u64, 600u64, 700u64]; // 7 items
    let sent = producer.send_adaptive(&data).unwrap();
    assert_eq!(sent, 7); // All 7 items sent (4 via SIMD, 3 via singles)
    
    let mut buffer = vec![0u64; 10];
    let received = consumer.recv_adaptive(&mut buffer);
    assert_eq!(received, 7); // All 7 items received adaptively
    assert_eq!(&buffer[..received], data.as_slice());
    
    // Legacy batch operations (all-or-nothing for exactly 4 items)
    let batch_data = vec![1u64, 2u64, 3u64, 4u64];
    producer.send_batch(&batch_data).unwrap();
    
    let mut batch_buffer = vec![0u64; 4];
    let count = consumer.recv_batch(&mut batch_buffer);
    assert_eq!(count, 4);
    
    // Single operations (still optimized)
    producer.send(500u64).unwrap();
    assert_eq!(consumer.recv(), Some(500u64));
}
```

## Performance Characteristics

### Benchmark Results

**Single-Threaded Performance:**
```
Scenario               Regular Queue    SIMD Queue     Improvement
------------------------------------------------------------
Single operations      41M ops/sec     46M ops/sec    +12%
Batch operations       -               18M ops/sec    N/A (4.5 ns/element)
```

**Multi-Threaded Performance:**
```
Thread Pairs    Regular Queue    SIMD Batch     Improvement
----------------------------------------------------------
1 pair          43M ops/sec     24M ops/sec    -44% (overhead)
2 pairs         31M ops/sec     31M ops/sec    ~0% (equal)  
4 pairs         17M ops/sec     32M ops/sec    +88%
8 pairs         12M ops/sec     28M ops/sec    +133%
```

**Key Insights:**
- SIMD excels in high-contention scenarios (4+ threads)
- Single-threaded batch operations have higher per-element overhead
- Best performance with u64 data in exactly 4-element batches

### Memory and CPU Requirements

**Memory Layout:**
- Minimum capacity: 16 elements (2x SIMD width)
- Cache-line aligned slots (64-byte alignment)
- Power-of-2 capacity requirement maintained

**CPU Requirements:**
- x86-64 with AVX2 support (u64x4 SIMD)
- ARM64 with NEON (future support)

## Adaptive Operations (Recommended)

The adaptive methods provide the best user experience by automatically optimizing between SIMD and single operations.

### Adaptive Send

```rust
// Automatically handles any amount of data optimally
let data = vec![1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64];
match producer.send_adaptive(&data) {
    Ok(sent) => println!("Sent {} items", sent),           // sent = 7 
    Err(unsent) => println!("Couldn't send: {:?}", unsent), // Queue full
}

// Behind the scenes:
// - Items 1,2,3,4: Processed as SIMD batch (1 vectorized operation)
// - Items 5,6,7:   Processed individually (3 single operations)
// - Result: All 7 items successfully sent
```

### Adaptive Receive

```rust
// Receives whatever is available, up to buffer size
let mut buffer = vec![0u64; 10];
let received = consumer.recv_adaptive(&mut buffer);
println!("Received {} items: {:?}", received, &buffer[..received]);

// Adaptive behavior:
// - If 8+ items available: Uses 2 SIMD operations (8 items) + singles for rest
// - If 4-7 items available: Uses 1 SIMD operation (4 items) + singles for rest  
// - If 1-3 items available: Uses only single operations
// - If 0 items available: Returns 0 immediately
```

### Comparison: Adaptive vs Legacy Batch

```rust
// Scenario: Queue has 3 items

// Legacy batch (all-or-nothing)
let mut buffer = vec![0u64; 4];
let received = consumer.recv_batch(&mut buffer);  
// received = 0 (nothing, because less than 4 items available)

// Adaptive (gets whatever is available)  
let mut buffer = vec![0u64; 4];
let received = consumer.recv_adaptive(&mut buffer);
// received = 3 (all available items retrieved)
```

## Advanced Usage Patterns

### Hybrid Processing Strategy

For mixed workloads, use both batch and single operations:

```rust
fn process_data_stream(producer: &SimdProducer<u64>, data: &[u64]) -> Result<(), ()> {
    let mut i = 0;
    
    // Process in SIMD batches when possible
    while i + 4 <= data.len() {
        let batch = &data[i..i+4];
        match producer.send_batch(batch) {
            Ok(sent) => i += sent,
            Err(_) => {
                // Queue full, fall back to single operations
                producer.send(data[i]).map_err(|_| ())?;
                i += 1;
            }
        }
    }
    
    // Handle remaining elements individually
    while i < data.len() {
        producer.send(data[i]).map_err(|_| ())?;
        i += 1;
    }
    
    Ok(())
}
```

### High-Throughput Consumer Pattern

```rust
fn consume_with_batching(consumer: &SimdConsumer<u64>) -> Vec<u64> {
    let mut results = Vec::new();
    let mut batch_buffer = vec![0u64; 4];
    
    loop {
        // Try batch receive first
        let batch_count = consumer.recv_batch(&mut batch_buffer);
        if batch_count > 0 {
            results.extend_from_slice(&batch_buffer[..batch_count]);
            continue;
        }
        
        // Fall back to single receive
        match consumer.recv() {
            Some(item) => results.push(item),
            None => break, // Queue empty
        }
    }
    
    results
}
```

### Multi-Producer Coordination

```rust
use std::thread;

fn spawn_simd_producers(queue: Arc<SimdMpmcQueue<u64>>, data_sets: Vec<Vec<u64>>) {
    let handles: Vec<_> = data_sets.into_iter().enumerate().map(|(id, data)| {
        let producer = SimdProducer::new(Arc::clone(&queue));
        
        thread::spawn(move || {
            for chunk in data.chunks(4) {
                if chunk.len() == 4 {
                    // Optimal: Full SIMD batch
                    while producer.send_batch(chunk).is_err() {
                        thread::yield_now(); // Wait for space
                    }
                } else {
                    // Partial batch: Use single operations
                    for &item in chunk {
                        while producer.send(item).is_err() {
                            thread::yield_now();
                        }
                    }
                }
            }
            println!("Producer {} completed", id);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Performance Optimization Tips

### 1. Batch Size Alignment

**Optimal:**
```rust
// Perfect alignment for u64x4 SIMD
let data = vec![1u64, 2u64, 3u64, 4u64];
producer.send_batch(&data).unwrap(); // Single SIMD operation
```

**Suboptimal:**
```rust
// Non-aligned batch sizes
let data = vec![1u64, 2u64, 3u64]; // Falls back to single operations
producer.send_batch(&data).unwrap();
```

### 2. Queue Sizing

**Recommended Capacities:**
```rust
// High contention: Larger capacity reduces blocking
let queue = SimdMpmcQueue::<u64>::new(1024); // Good for 8+ threads

// Low contention: Smaller capacity improves cache locality  
let queue = SimdMpmcQueue::<u64>::new(64);   // Good for 2-4 threads
```

### 3. Memory Access Patterns

**Cache-Friendly:**
```rust
// Process data in sequential batches
for chunk in data.chunks(4) {
    producer.send_batch(chunk)?;
}
```

**Cache-Unfriendly:**
```rust
// Random access patterns hurt SIMD performance
for &index in random_indices {
    producer.send(data[index])?; // Consider regular queue
}
```

### 4. Thread Scaling

**Optimal Thread Distribution:**
```rust
let cpu_cores = num_cpus::get();
let optimal_threads = std::cmp::min(cpu_cores, 8); // Diminishing returns after 8

// Create balanced producer/consumer pairs
for _ in 0..optimal_threads {
    spawn_producer();
    spawn_consumer(); 
}
```

## Compilation and Build Options

### Feature Flags

```toml
[features]
default = ["simd"]
simd = []

# Optional: Disable SIMD for stable Rust
# default = []
```

### Build Commands

```bash
# Development (with SIMD)
cargo build --features simd

# Release optimization
cargo build --release --features simd

# Benchmarking
cargo bench --features simd

# Testing
cargo test --features simd
```

### Conditional Compilation

```rust
#[cfg(feature = "simd")]
use mpmc_std::simd_queue::{SimdMpmcQueue, SimdProducer, SimdConsumer};

#[cfg(not(feature = "simd"))]
use mpmc_std::{MpmcQueue as SimdMpmcQueue, Producer as SimdProducer, Consumer as SimdConsumer};

// Code works with both variants
fn generic_processing() {
    let queue = Arc::new(SimdMpmcQueue::<u64>::new(64));
    let producer = SimdProducer::new(Arc::clone(&queue));
    // ...
}
```

## Troubleshooting

### Common Issues

**1. Compilation Errors:**
```
error[E0658]: use of unstable library feature `portable_simd`
```
**Solution:** Ensure nightly toolchain: `rustup default nightly`

**2. Performance Lower Than Expected:**
- Check batch sizes are multiples of 4
- Verify CPU supports AVX2 instructions
- Measure with high-contention scenarios (4+ threads)

**3. Queue Capacity Issues:**
```
assertion failed: capacity > 0
```
**Solution:** SIMD queue has minimum capacity of 8 elements

### Performance Debugging

```rust
// Add timing measurements
use std::time::Instant;

let start = Instant::now();
for batch in data.chunks(4) {
    producer.send_batch(batch)?;
}
let duration = start.elapsed();

println!("Throughput: {:.0} ops/sec", 
         (data.len() as f64) / duration.as_secs_f64());
```

### Profiling Tools

**CPU Profiling:**
```bash
# Profile SIMD instruction usage
perf record -e cycles,instructions,cache-misses cargo bench --features simd
perf report
```

**Memory Analysis:**
```bash
# Check cache performance
valgrind --tool=cachegrind cargo run --features simd --example simd_benchmark
```

## When to Use SIMD vs Regular Queue

### Choose SIMD Queue When:
- Processing u64 numeric data
- Batch sizes are multiples of 4
- High-contention scenarios (4+ threads)
- Latency-sensitive applications with numeric workloads
- CPU supports AVX2+ instructions

### Choose Regular Queue When:
- Mixed data types or generic types
- Variable or small batch sizes
- Low-contention scenarios (1-2 threads)  
- Stable Rust requirement
- Memory-constrained environments

### Migration Strategy

```rust
// Gradual migration approach
fn create_optimal_queue<T>() -> Box<dyn QueueTrait<T>> {
    #[cfg(feature = "simd")]
    if std::mem::size_of::<T>() == 8 && is_numeric::<T>() {
        Box::new(SimdMpmcQueue::<u64>::new(capacity))
    } else {
        Box::new(MpmcQueue::<T>::new(capacity))
    }
    
    #[cfg(not(feature = "simd"))]
    Box::new(MpmcQueue::<T>::new(capacity))
}
```

This guide provides comprehensive coverage of SIMD optimization techniques for achieving maximum throughput with the mpmc-std library.