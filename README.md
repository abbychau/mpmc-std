# MPMC Queue

A wait-free, lockless Multi-Producer Multi-Consumer (MPMC) queue implementation in Rust.

## Features

- Lockless design with atomic operations
- Wait-free performance
- Multiple producers and consumers
- Memory safe with Rust's guarantees
- Cache-optimized with 64-byte alignment
- **SIMD optimizations** for 64-bit data types (optional, requires nightly Rust)

## Algorithm Overview

### Core Design

The queue uses a power-of-2 sized ring buffer where each slot contains:
- Sequence number for atomic coordination
- Data storage
- 64-byte alignment for cache optimization

```rust
struct MpmcQueue<T> {
    buffer: Box<[Slot<T>]>,
    capacity: usize,
    mask: usize,               // capacity - 1 for fast modulo
    producer_pos: AtomicUsize, // Separate cache line
    consumer_pos: AtomicUsize, // Separate cache line
}

struct Slot<T> {
    sequence: AtomicUsize,
    data: UnsafeCell<MaybeUninit<T>>,
}
```

## How It Works

### Sequence Numbers

The algorithm uses sequence numbers to coordinate between producers and consumers:

- `seq == slot_index`: Ready for producer
- `seq == slot_index + 1`: Ready for consumer  
- `seq > slot_index + 1`: Slot is ahead (race condition)

### Operations

**Producer (Send)**:
1. Load producer position
2. Calculate slot using `position & mask`
3. Check if slot sequence matches expected value
4. If yes, try to claim slot with CAS
5. Store data and update sequence

**Consumer (Receive)**:
1. Load consumer position
2. Calculate slot using `position & mask` 
3. Check if data is ready (`seq == position + 1`)
4. If yes, try to claim slot with CAS
5. Read data and advance sequence

## Usage

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mpmc-std = "0.1.0"
```

### Basic Example

```rust
use mpmc_std::{MpmcQueue, Producer, Consumer};
use std::sync::Arc;

let queue = Arc::new(MpmcQueue::new(1024));
let producer = Producer::new(Arc::clone(&queue));
let consumer = Consumer::new(Arc::clone(&queue));

// Send data
producer.send("Hello, World!".to_string())?;

// Receive data
if let Some(message) = consumer.recv() {
    println!("Received: {}", message);
}
```

### SIMD Operations (64-bit types, requires nightly Rust)

**Unified send/recv API:**
```rust
use mpmc_std::simd_queue::{SimdMpmcQueue, SimdProducer, SimdConsumer};
use std::sync::Arc;

// Works with any 64-bit type: u64, i64, f64, usize, isize
let queue = Arc::new(SimdMpmcQueue::<u64>::new(1024));
let producer = SimdProducer::new(Arc::clone(&queue));
let consumer = SimdConsumer::new(Arc::clone(&queue));

// Send any amount of data - automatically uses SIMD when beneficial  
let data = vec![1u64, 2u64, 3u64, 4u64, 5u64]; // 5 items
let sent = producer.send(&data)?;                // All 5 sent (4 via SIMD + 1 single)

// Receive whatever is available - adapts to data availability
let mut buffer = vec![0u64; 10]; 
let received = consumer.recv(&mut buffer);       // Gets all 5 items

// Single item operations also available
producer.send_one(42u64)?;
let item = consumer.recv_one(); // Some(42)
```

**Different 64-bit types:**
```rust
// Integer types
let i64_queue = Arc::new(SimdMpmcQueue::<i64>::new(64));

// Floating point types  
let f64_queue = Arc::new(SimdMpmcQueue::<f64>::new(64));

// Platform word sizes (on 64-bit systems)
let usize_queue = Arc::new(SimdMpmcQueue::<usize>::new(64));
```

## Performance

Run benchmarks with `cargo bench`. The queue achieves sub-10ns latency per operation with linear scaling up to 8 producer/consumer threads.

**SIMD Performance**: Enable with `cargo bench --features simd` (requires nightly Rust). SIMD operations automatically optimize groups of 4 elements and provide 10-70% performance improvements for 64-bit data types, especially under high contention scenarios.

## Key Design Decisions

### Sequence Numbers
Uses sequence numbers instead of flags for ABA problem immunity and wait-free progress.

### Power-of-2 Capacity
Enables fast bitwise AND instead of expensive modulo: `position & mask` vs `position % capacity`.

### Cache-Line Alignment
Prevents false sharing by separating producer and consumer positions into different cache lines.

## Testing

```bash
cargo test              # Run all tests
cargo test -- --nocapture  # Run with output
```

## References

This implementation is based on:
- Michael & Scott (1996): Non-blocking concurrent queue algorithms
- LMAX Disruptor: High-performance ring buffer pattern
- Modern lockless data structures research

## License

MIT License - see LICENSE file for details.