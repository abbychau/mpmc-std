# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a high-performance Multi-Producer Multi-Consumer (MPMC) queue implementation in Rust. The queue uses a wait-free, lockless algorithm based on sequence numbers and atomic operations, achieving sub-10ns latency per operation with linear scaling up to 8 producer/consumer threads.

**SIMD-Optimized Version**: Includes SIMD (Single Instruction Multiple Data) optimizations using Rust's std::simd for batch operations on u64 data, providing additional performance improvements for vectorizable workloads.

## Core Architecture

The implementation consists of three main components:

### 1. MpmcQueue Core (`src/lib.rs`)
- **Ring Buffer**: Power-of-2 sized buffer with cache-line aligned slots
- **Sequence Numbers**: Each slot uses atomic sequence numbers for coordination instead of flags
- **Separate Positions**: Producer and consumer positions are cache-line aligned to prevent false sharing
- **Wait-Free Operations**: Both `send()` and `recv()` are wait-free with no artificial retry limits

### 2. Producer/Consumer Handles
- **Producer**: Multiple producers can send items concurrently via `Producer::new()`
- **Consumer**: Multiple consumers can receive items concurrently via `Consumer::new()`
- **Arc-based**: Both handles use `Arc<MpmcQueue<T>>` for shared ownership
- **SIMD Variants**: `SimdProducer` and `SimdConsumer` for batch operations (requires nightly Rust)

### 3. Sequence-Based Coordination
- `seq == slot_index`: Slot ready for producer
- `seq == slot_index + 1`: Slot ready for consumer
- `seq > slot_index + 1`: Slot is ahead (race condition handling)

## Development Commands

### Building and Testing
```bash
cargo build              # Build the library
cargo build --features simd  # Build with SIMD optimizations (requires nightly)
cargo run                # Run the main demo
cargo run --features simd --example simd_benchmark  # Run SIMD performance comparison
cargo test               # Run all tests
cargo test --features simd  # Run tests including SIMD tests
cargo test -- --nocapture  # Run tests with output
cargo test test_name     # Run a specific test
```

### SIMD Features (Requires Nightly Rust)
```bash
rustup default nightly   # Switch to nightly toolchain
cargo build --features simd  # Enable SIMD optimizations
cargo test --features simd   # Test SIMD functionality
cargo bench --features simd simd_bench  # Run SIMD benchmarks
```

### Benchmarking
```bash
cargo bench              # Run all benchmarks (generates HTML reports)
cargo bench mpmc_bench   # Run specific benchmark suite
cargo bench --features simd  # Run all benchmarks including SIMD
cargo bench --features simd simd_bench  # Run only SIMD benchmarks
cargo run --example simple_benchmark    # Run simple performance test
cargo run --features simd --example simd_benchmark  # Run SIMD comparison
```

### Documentation Generation
```bash
./copy_benchmarks.sh     # Generate docs with benchmark results
./preview_docs.sh        # Start local HTTP server for docs (port 8080)
./preview_docs.sh 3000   # Start server on custom port
npm run docs             # Alternative: run copy_benchmarks.sh
npm run preview          # Alternative: run preview_docs.sh
node scripts/markdown-converter.js input.md  # Convert markdown to HTML
```

## Key Implementation Details

### Memory Layout
- Each slot is 64-byte cache-line aligned to prevent false sharing
- Producer and consumer positions are in separate cache lines
- Buffer size is always rounded up to next power-of-2 for efficient masking
- SIMD queue enforces minimum capacity for vectorized operations (16+ elements)

### Algorithm Characteristics
- **Wait-Free**: No artificial retry limits or blocking operations
- **ABA-Safe**: Sequence numbers prevent ABA problems without hazard pointers
- **Cache Optimized**: Designed to minimize cache line bouncing
- **Capacity**: Must be power-of-2; automatically rounded up if not
- **SIMD Batch Processing**: Processes 4 u64 elements simultaneously using 256-bit SIMD instructions

### SIMD Optimizations
- **Vectorized Sequence Checking**: Uses SIMD to check multiple slot availability simultaneously
- **Batch Operations**: `send_batch()` and `recv_batch()` for processing 4-element u64 arrays
- **Hybrid Fallback**: Falls back to single operations for non-aligned batch sizes
- **Target Architecture**: Optimized for x86-64 AVX2+ (u64x4 SIMD width)

### Performance Characteristics
- **Regular Queue**: ~1.8B operations/second, ~8.9ns latency per operation
- **SIMD Batch**: Up to 1.7x speedup in high-contention scenarios (4+ thread pairs)
- **SIMD Single**: ~10-30% performance improvement for single operations
- **Scaling**: Linear up to 8 producer/consumer threads
- **Contention**: Handles high contention scenarios with 16+ threads
- **SIMD Best Use**: u64 numeric data, batch sizes aligned to 4 elements

## Testing Strategy

### Unit Tests (`src/main.rs:70-377`)
- Basic send/recv functionality
- Multiple producer/consumer scenarios
- Bounded capacity verification
- Empty queue behavior
- High contention testing (8 producers + 8 consumers)
- **SIMD Tests**: Batch operations, mixed single/batch operations, edge cases

### Benchmark Suite (`benches/mpmc_bench.rs`)
- **Single-threaded throughput**: Various queue capacities (64, 256, 1024, 4096)
- **Multi-producer single-consumer**: Scaling with 1, 2, 4, 8 producers
- **Single-producer multi-consumer**: Scaling with 1, 2, 4, 8 consumers
- **Multi-producer multi-consumer**: Balanced thread pairs (1, 2, 4, 8)
- **Latency measurement**: Send/receive operation timing
- **Contention benchmark**: 16 threads on small queues (16, 64, 256)

### SIMD Benchmark Suite (`benches/simd_bench.rs`)
- **SIMD vs Regular comparison**: Batch vs single operation performance
- **Multi-threaded SIMD**: SIMD batch operations under contention
- **Batch size optimization**: Finding optimal SIMD batch sizes (1-32 elements)
- **Latency comparison**: SIMD batch vs single vs regular queue latencies

## Common Development Tasks

When working on performance improvements:
1. Run `cargo bench` to establish baseline
2. Make changes to `src/lib.rs` or `src/simd_queue.rs`
3. Run `cargo bench` again to measure impact
4. For SIMD changes, also run `cargo bench --features simd simd_bench`
5. Use `./copy_benchmarks.sh` to generate comparison reports

When adding new features:
1. Add functionality to `MpmcQueue` struct or `SimdMpmcQueue` for SIMD features
2. Update `Producer`/`Consumer` handles and `SimdProducer`/`SimdConsumer` if needed
3. Add comprehensive tests in `src/main.rs` (include SIMD tests under `#[cfg(feature = "simd")]`)
4. Add benchmark cases in `benches/mpmc_bench.rs` or `benches/simd_bench.rs`

When working with SIMD:
1. Ensure nightly Rust toolchain: `rustup default nightly`
2. Test both regular and SIMD builds: `cargo test` and `cargo test --features simd`
3. Use `cargo run --features simd --example simd_benchmark` for quick performance comparisons
4. SIMD optimizations work best for u64 numeric data in multiples of 4

## Documentation Structure

- `README.md`: User-facing documentation with algorithm overview
- `docs/ALGORITHM_DIAGRAMS.md`: Visual algorithm explanations
- `docs/IMPLEMENTATION_NOTES.md`: Technical implementation details
- `docs/COMPARATIVE_ANALYSIS.md`: Performance comparisons with other queue implementations
- Generated HTML docs in `docs/` directory after running `./copy_benchmarks.sh`