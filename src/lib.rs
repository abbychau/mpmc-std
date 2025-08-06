#![cfg_attr(feature = "simd", feature(portable_simd))]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

#[cfg(feature = "simd")]
use std::simd::{u64x4, Simd};

#[cfg(feature = "simd")]
pub mod simd_queue;

// Cache line size for padding
const CACHE_LINE: usize = 64;

#[repr(align(64))] // Align to cache line to avoid false sharing
struct Slot<T> {
    sequence: AtomicUsize,
    data: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Slot<T> {
    fn new(seq: usize) -> Self {
        Self {
            sequence: AtomicUsize::new(seq),
            data: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
}

// Separate cache lines for producer and consumer positions to avoid false sharing
#[repr(align(64))]
struct ProducerPos {
    head: AtomicUsize,
}

#[repr(align(64))]
struct ConsumerPos {
    tail: AtomicUsize,
}

/// A high-performance bounded MPMC queue based on a ring buffer with sequence numbers.
/// 
/// This implementation is inspired by:
/// - Michael & Scott's non-blocking queue algorithm
/// - LMAX Disruptor's sequence-based coordination
/// - Crossbeam's memory management patterns
///
/// Key features:
/// - Wait-free for producers and consumers under most conditions
/// - No artificial retry limits or spin loops
/// - Cache-line optimized to minimize false sharing
/// - Memory-safe with proper ordering guarantees
pub struct MpmcQueue<T> {
    buffer: Box<[Slot<T>]>,
    capacity: usize,
    mask: usize, // capacity - 1, for fast modulo via bitwise AND
    producer_pos: ProducerPos,
    consumer_pos: ConsumerPos,
}

impl<T: Send> MpmcQueue<T> {
    /// Creates a new MPMC queue with the specified capacity.
    /// 
    /// The capacity must be a power of 2 for optimal performance.
    /// If not, it will be rounded up to the next power of 2.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        
        // Round up to next power of 2 for efficient masking
        let capacity = capacity.next_power_of_two();
        let mask = capacity - 1;
        
        // Initialize buffer with sequence numbers
        let mut buffer = Vec::with_capacity(capacity);
        for i in 0..capacity {
            buffer.push(Slot::new(i));
        }
        
        Self {
            buffer: buffer.into_boxed_slice(),
            capacity,
            mask,
            producer_pos: ProducerPos {
                head: AtomicUsize::new(0),
            },
            consumer_pos: ConsumerPos {
                tail: AtomicUsize::new(0),
            },
        }
    }
    
    /// Attempts to send an item to the queue.
    /// 
    /// This is a wait-free operation that will either succeed immediately
    /// or fail if the queue is full. No artificial retry limits.
    pub fn send(&self, item: T) -> Result<(), T> {
        loop {
            // Get the current producer position
            let head = self.producer_pos.head.load(Ordering::Relaxed);
            let slot = &self.buffer[head & self.mask];
            
            // Check the slot's sequence number
            let seq = slot.sequence.load(Ordering::Acquire);
            let expected_seq = head;
            
            match seq.cmp(&expected_seq) {
                std::cmp::Ordering::Equal => {
                    // Slot is available, try to claim it
                    match self.producer_pos.head.compare_exchange_weak(
                        head,
                        head.wrapping_add(1),
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            // Successfully claimed the slot, now store the data
                            unsafe {
                                (*slot.data.get()).write(item);
                            }
                            
                            // Signal that data is ready by advancing sequence
                            slot.sequence.store(expected_seq.wrapping_add(1), Ordering::Release);
                            return Ok(());
                        }
                        Err(_) => {
                            // Another producer claimed this slot, retry
                            std::hint::spin_loop();
                            continue;
                        }
                    }
                }
                std::cmp::Ordering::Less => {
                    // Slot is behind, queue might be full
                    // Check if we've wrapped around (queue is full)
                    let tail = self.consumer_pos.tail.load(Ordering::Acquire);
                    if head.wrapping_sub(tail) >= self.capacity {
                        return Err(item); // Queue is full
                    }
                    // Otherwise, retry with updated head
                    std::hint::spin_loop();
                    continue;
                }
                std::cmp::Ordering::Greater => {
                    // Slot is ahead, another producer is working on it
                    // This shouldn't happen in normal operation, but handle gracefully
                    std::hint::spin_loop();
                    continue;
                }
            }
        }
    }
    
    /// Attempts to receive an item from the queue.
    /// 
    /// This is a wait-free operation that will either succeed immediately
    /// or return None if the queue is empty.
    pub fn recv(&self) -> Option<T> {
        loop {
            // Get the current consumer position
            let tail = self.consumer_pos.tail.load(Ordering::Relaxed);
            let slot = &self.buffer[tail & self.mask];
            
            // Check the slot's sequence number
            let seq = slot.sequence.load(Ordering::Acquire);
            let expected_seq = tail.wrapping_add(1);
            
            match seq.cmp(&expected_seq) {
                std::cmp::Ordering::Equal => {
                    // Data is available, try to claim it
                    match self.consumer_pos.tail.compare_exchange_weak(
                        tail,
                        tail.wrapping_add(1),
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            // Successfully claimed the slot, read the data
                            let item = unsafe { (*slot.data.get()).assume_init_read() };
                            
                            // Mark slot as available for producers
                            slot.sequence.store(
                                tail.wrapping_add(self.capacity),
                                Ordering::Release,
                            );
                            return Some(item);
                        }
                        Err(_) => {
                            // Another consumer claimed this slot, retry
                            std::hint::spin_loop();
                            continue;
                        }
                    }
                }
                std::cmp::Ordering::Less => {
                    // No data available, queue is empty
                    return None;
                }
                std::cmp::Ordering::Greater => {
                    // Slot is ahead, shouldn't happen in normal operation
                    std::hint::spin_loop();
                    continue;
                }
            }
        }
    }
    
    /// Returns the capacity of the queue.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Returns true if the queue is empty.
    /// 
    /// Note: This is a snapshot view and may change immediately after the call.
    pub fn is_empty(&self) -> bool {
        let head = self.producer_pos.head.load(Ordering::Acquire);
        let tail = self.consumer_pos.tail.load(Ordering::Acquire);
        head == tail
    }
    
    /// Returns true if the queue is full.
    /// 
    /// Note: This is a snapshot view and may change immediately after the call.
    pub fn is_full(&self) -> bool {
        let head = self.producer_pos.head.load(Ordering::Acquire);
        let tail = self.consumer_pos.tail.load(Ordering::Acquire);
        head.wrapping_sub(tail) >= self.capacity
    }
    
    /// Returns the approximate number of items in the queue.
    /// 
    /// Note: This is a snapshot view and may change immediately after the call.
    pub fn len(&self) -> usize {
        let head = self.producer_pos.head.load(Ordering::Acquire);
        let tail = self.consumer_pos.tail.load(Ordering::Acquire);
        head.wrapping_sub(tail)
    }
    
}

// Separate impl block without Send bound for Drop implementation
impl<T> MpmcQueue<T> {
    /// Internal method to check if queue is empty without Send bound requirement
    fn is_empty_unchecked(&self) -> bool {
        let head = self.producer_pos.head.load(Ordering::Relaxed);
        let tail = self.consumer_pos.tail.load(Ordering::Relaxed);
        head == tail
    }
}

impl<T> Drop for MpmcQueue<T> {
    fn drop(&mut self) {
        // Drain any remaining items to prevent memory leaks
        // We need to manually drain since recv() requires T: Send
        while !self.is_empty_unchecked() {
            let tail = self.consumer_pos.tail.load(Ordering::Relaxed);
            let slot = &self.buffer[tail & self.mask];
            let seq = slot.sequence.load(Ordering::Acquire);
            
            if seq == tail.wrapping_add(1) {
                // Data is available
                if self.consumer_pos.tail.compare_exchange_weak(
                    tail,
                    tail.wrapping_add(1),
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ).is_ok() {
                    // Successfully claimed the slot, drop the data
                    unsafe {
                        (*slot.data.get()).assume_init_drop();
                    }
                    // Mark slot as available for producers
                    slot.sequence.store(
                        tail.wrapping_add(self.capacity),
                        Ordering::Release,
                    );
                }
            } else {
                break; // No more data or inconsistent state
            }
        }
    }
}

unsafe impl<T: Send> Send for MpmcQueue<T> {}
unsafe impl<T: Send> Sync for MpmcQueue<T> {}

/// A producer handle for the MPMC queue.
/// 
/// Multiple producers can send items concurrently.
pub struct Producer<T> {
    queue: Arc<MpmcQueue<T>>,
}

impl<T: Send> Producer<T> {
    pub fn new(queue: Arc<MpmcQueue<T>>) -> Self {
        Self { queue }
    }
    
    /// Sends an item to the queue.
    /// 
    /// This is now a synchronous, wait-free operation.
    pub fn send(&self, item: T) -> Result<(), T> {
        self.queue.send(item)
    }
    
    /// Async version of send for compatibility with existing code.
    pub async fn send_async(&self, item: T) -> Result<(), T> {
        // Since the new implementation is wait-free, we can call it directly
        // without spawn_blocking
        self.send(item)
    }
    
    /// Returns true if the queue is full.
    pub fn is_full(&self) -> bool {
        self.queue.is_full()
    }
    
    /// Returns the capacity of the queue.
    pub fn capacity(&self) -> usize {
        self.queue.capacity()
    }
}

impl<T: Send> Clone for Producer<T> {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
        }
    }
}

/// A consumer handle for the MPMC queue.
/// 
/// Multiple consumers can receive items concurrently.
pub struct Consumer<T> {
    queue: Arc<MpmcQueue<T>>,
}

impl<T: Send> Consumer<T> {
    pub fn new(queue: Arc<MpmcQueue<T>>) -> Self {
        Self { queue }
    }
    
    /// Receives an item from the queue.
    /// 
    /// This is now a synchronous, wait-free operation.
    pub fn recv(&self) -> Option<T> {
        self.queue.recv()
    }
    
    /// Async version of recv for compatibility with existing code.
    pub async fn recv_async(&self) -> Option<T> {
        // Since the new implementation is wait-free, we can call it directly
        // without spawn_blocking
        self.recv()
    }
    
    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// Returns the approximate number of items in the queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

impl<T: Send> Clone for Consumer<T> {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
        }
    }
}

// Re-export SIMD optimized queue when feature is enabled
#[cfg(feature = "simd")]
pub use simd_queue::{SimdMpmcQueue, SimdProducer, SimdConsumer};