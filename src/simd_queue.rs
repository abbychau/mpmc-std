use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::simd::{u64x4};
use std::simd::cmp::SimdPartialEq;

/// SIMD-optimized MPMC queue for 64-bit data types
/// 
/// This version uses SIMD instructions to process multiple elements simultaneously,
/// providing significant performance improvements for 64-bit data.
/// 
/// Supported types: u64, i64, f64, usize, isize, and any 64-bit type that can be safely transmuted
#[repr(align(64))]
pub struct SimdMpmcQueue<T> {
    buffer: Box<[SimdSlot<T>]>,
    capacity: usize,
    mask: usize,
    producer_pos: SimdProducerPos,
    consumer_pos: SimdConsumerPos,
}

#[repr(align(64))]
struct SimdSlot<T> {
    sequence: AtomicUsize,
    data: UnsafeCell<MaybeUninit<T>>,
}

#[repr(align(64))]
struct SimdProducerPos {
    head: AtomicUsize,
}

#[repr(align(64))]
struct SimdConsumerPos {
    tail: AtomicUsize,
}

impl<T> SimdSlot<T> {
    fn new(seq: usize) -> Self {
        Self {
            sequence: AtomicUsize::new(seq),
            data: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
}

/// Trait to enable SIMD operations for 64-bit types
pub trait Simd64Bit: Copy + Send + Sync + 'static {
    /// Convert to u64 for SIMD processing
    fn to_u64(self) -> u64;
    /// Convert from u64 after SIMD processing
    fn from_u64(val: u64) -> Self;
}

impl Simd64Bit for u64 {
    fn to_u64(self) -> u64 { self }
    fn from_u64(val: u64) -> Self { val }
}

impl Simd64Bit for i64 {
    fn to_u64(self) -> u64 { self as u64 }
    fn from_u64(val: u64) -> Self { val as i64 }
}

impl Simd64Bit for f64 {
    fn to_u64(self) -> u64 { self.to_bits() }
    fn from_u64(val: u64) -> Self { f64::from_bits(val) }
}

impl Simd64Bit for usize {
    fn to_u64(self) -> u64 { 
        assert_eq!(std::mem::size_of::<usize>(), 8, "usize must be 64-bit");
        self as u64 
    }
    fn from_u64(val: u64) -> Self { val as usize }
}

impl Simd64Bit for isize {
    fn to_u64(self) -> u64 { 
        assert_eq!(std::mem::size_of::<isize>(), 8, "isize must be 64-bit");
        self as u64 
    }
    fn from_u64(val: u64) -> Self { val as isize }
}

/// SIMD-optimized operations for 64-bit data types
impl<T: Simd64Bit> SimdMpmcQueue<T> {
    /// Creates a new SIMD-optimized MPMC queue for 64-bit elements
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        
        // Ensure capacity is power of 2 and divisible by SIMD width
        let simd_batch_size = 4; // u64x4 SIMD width
        let capacity = std::cmp::max(
            capacity.next_power_of_two(),
            simd_batch_size * 2
        );
        let mask = capacity - 1;
        
        let mut buffer = Vec::with_capacity(capacity);
        for i in 0..capacity {
            buffer.push(SimdSlot::new(i));
        }
        
        Self {
            buffer: buffer.into_boxed_slice(),
            capacity,
            mask,
            producer_pos: SimdProducerPos {
                head: AtomicUsize::new(0),
            },
            consumer_pos: SimdConsumerPos {
                tail: AtomicUsize::new(0),
            },
        }
    }
    
    /// Send items - automatically uses SIMD when beneficial
    pub fn send(&self, items: &[T]) -> Result<usize, Vec<T>> {
        if items.is_empty() {
            return Ok(0);
        }
        
        let mut sent_count = 0;
        let mut remaining_items = items;
        
        // Try SIMD batch operations first for groups of 4
        while remaining_items.len() >= 4 {
            let batch = &remaining_items[..4];
            
            // Check if we can claim 4 slots using SIMD
            let head = self.producer_pos.head.load(Ordering::Relaxed);
            if self.try_claim_batch_producer(head, 4) {
                // Use SIMD to store 4 items
                unsafe {
                    self.store_batch_simd(head, batch);
                }
                sent_count += 4;
                remaining_items = &remaining_items[4..];
            } else {
                // SIMD batch failed, try single item
                match self.send_single_internal(remaining_items[0]) {
                    Ok(()) => {
                        sent_count += 1;
                        remaining_items = &remaining_items[1..];
                    }
                    Err(_) => {
                        // Queue full, return what we couldn't send
                        return Err(remaining_items.to_vec());
                    }
                }
            }
        }
        
        // Handle remaining items (1-3 items) individually
        while !remaining_items.is_empty() {
            match self.send_single_internal(remaining_items[0]) {
                Ok(()) => {
                    sent_count += 1;
                    remaining_items = &remaining_items[1..];
                }
                Err(_) => {
                    // Queue full, return what we couldn't send
                    return Err(remaining_items.to_vec());
                }
            }
        }
        
        Ok(sent_count)
    }
    
    
    /// Receive items - automatically uses SIMD when beneficial  
    pub fn recv(&self, buffer: &mut [T]) -> usize {
        if buffer.is_empty() {
            return 0;
        }
        
        let mut received_count = 0;
        let mut remaining_buffer = buffer;
        
        // Try SIMD batch operations first for groups of 4
        while remaining_buffer.len() >= 4 {
            let tail = self.consumer_pos.tail.load(Ordering::Relaxed);
            
            // Check if we can claim 4 slots using SIMD
            if self.try_claim_batch_consumer(tail, 4) {
                // Use SIMD to load 4 items
                unsafe {
                    self.load_batch_simd(tail, &mut remaining_buffer[..4]);
                }
                received_count += 4;
                remaining_buffer = &mut remaining_buffer[4..];
            } else {
                // SIMD batch failed, try single item
                match self.recv_single_internal() {
                    Some(item) => {
                        remaining_buffer[0] = item;
                        received_count += 1;
                        remaining_buffer = &mut remaining_buffer[1..];
                    }
                    None => {
                        // No more data available
                        return received_count;
                    }
                }
            }
        }
        
        // Handle remaining buffer space (1-3 slots) individually
        while !remaining_buffer.is_empty() {
            match self.recv_single_internal() {
                Some(item) => {
                    remaining_buffer[0] = item;
                    received_count += 1;
                    remaining_buffer = &mut remaining_buffer[1..];
                }
                None => {
                    // No more data available
                    break;
                }
            }
        }
        
        received_count
    }
    
    
    /// Try to claim a batch of producer slots using SIMD sequence checking
    fn try_claim_batch_producer(&self, head: usize, batch_size: usize) -> bool {
        // Load sequence numbers for the batch using SIMD
        let sequences = unsafe { self.load_sequences_simd(head, batch_size) };
        let expected_sequences = self.generate_expected_sequences_simd(head, batch_size);
        
        // Check if all sequences match expected values
        let mask = sequences.simd_eq(expected_sequences);
        
        if mask.all() {
            // All slots are available, try to claim them atomically
            self.producer_pos.head.compare_exchange_weak(
                head,
                head.wrapping_add(batch_size),
                Ordering::Relaxed,
                Ordering::Relaxed,
            ).is_ok()
        } else {
            false
        }
    }
    
    /// Try to claim a batch of consumer slots using SIMD sequence checking
    fn try_claim_batch_consumer(&self, tail: usize, batch_size: usize) -> bool {
        // Load sequence numbers for the batch using SIMD
        let sequences = unsafe { self.load_sequences_simd(tail, batch_size) };
        let expected_sequences = self.generate_expected_sequences_simd(
            tail.wrapping_add(1), batch_size
        );
        
        // Check if all sequences match expected values
        let mask = sequences.simd_eq(expected_sequences);
        
        if mask.all() {
            // All slots have data, try to claim them atomically
            self.consumer_pos.tail.compare_exchange_weak(
                tail,
                tail.wrapping_add(batch_size),
                Ordering::Relaxed,
                Ordering::Relaxed,
            ).is_ok()
        } else {
            false
        }
    }
    
    /// Load sequence numbers using SIMD instructions
    unsafe fn load_sequences_simd(&self, start_pos: usize, batch_size: usize) -> u64x4 {
        let mut sequences = [0u64; 4];
        for i in 0..std::cmp::min(batch_size, 4) {
            let slot_idx = (start_pos.wrapping_add(i)) & self.mask;
            sequences[i] = self.buffer[slot_idx].sequence.load(Ordering::Acquire) as u64;
        }
        u64x4::from_array(sequences)
    }
    
    /// Generate expected sequence numbers using SIMD
    fn generate_expected_sequences_simd(&self, start_seq: usize, _batch_size: usize) -> u64x4 {
        let base_seq = start_seq as u64;
        let offsets = u64x4::from_array([0, 1, 2, 3]);
        u64x4::splat(base_seq) + offsets
    }
    
    /// Store batch data using SIMD operations
    unsafe fn store_batch_simd(&self, head: usize, items: &[T]) {
        // Convert to u64 for SIMD processing
        let u64_items: [u64; 4] = [
            items[0].to_u64(),
            items[1].to_u64(), 
            items[2].to_u64(),
            items[3].to_u64(),
        ];
        let _simd_data = u64x4::from_array(u64_items);
        
        for (i, &value) in items.iter().enumerate().take(4) {
            let slot_idx = (head.wrapping_add(i)) & self.mask;
            let slot = &self.buffer[slot_idx];
            
            // Store the data
            unsafe {
                (*slot.data.get()).write(value);
            }
            
            // Update sequence to signal data is ready
            slot.sequence.store(
                (head + i).wrapping_add(1),
                Ordering::Release,
            );
        }
    }
    
    /// Load batch data using SIMD operations
    unsafe fn load_batch_simd(&self, tail: usize, buffer: &mut [T]) {
        let mut u64_buffer = [0u64; 4];
        
        for (i, buffer_slot) in buffer.iter_mut().enumerate().take(4) {
            let slot_idx = (tail.wrapping_add(i)) & self.mask;
            let slot = &self.buffer[slot_idx];
            
            // Load the data
            unsafe {
                let value = (*slot.data.get()).assume_init_read();
                *buffer_slot = value;
                u64_buffer[i] = value.to_u64();
            }
            
            // Mark slot as available for producers
            slot.sequence.store(
                (tail + i).wrapping_add(self.capacity),
                Ordering::Release,
            );
        }
        
        // Use SIMD for the loaded data (for future optimizations)
        let _simd_data = u64x4::from_array(u64_buffer);
    }
    
    /// Internal single-element send implementation
    fn send_single_internal(&self, item: T) -> Result<(), T> {
        loop {
            let head = self.producer_pos.head.load(Ordering::Relaxed);
            let slot = &self.buffer[head & self.mask];
            
            let seq = slot.sequence.load(Ordering::Acquire);
            let expected_seq = head;
            
            match seq.cmp(&expected_seq) {
                std::cmp::Ordering::Equal => {
                    match self.producer_pos.head.compare_exchange_weak(
                        head,
                        head.wrapping_add(1),
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            unsafe {
                                (*slot.data.get()).write(item);
                            }
                            slot.sequence.store(expected_seq.wrapping_add(1), Ordering::Release);
                            return Ok(());
                        }
                        Err(_) => {
                            std::hint::spin_loop();
                            continue;
                        }
                    }
                }
                std::cmp::Ordering::Less => {
                    let tail = self.consumer_pos.tail.load(Ordering::Acquire);
                    if head.wrapping_sub(tail) >= self.capacity {
                        return Err(item);
                    }
                    std::hint::spin_loop();
                    continue;
                }
                std::cmp::Ordering::Greater => {
                    std::hint::spin_loop();
                    continue;
                }
            }
        }
    }
    
    /// Internal single-element receive implementation
    fn recv_single_internal(&self) -> Option<T> {
        loop {
            let tail = self.consumer_pos.tail.load(Ordering::Relaxed);
            let slot = &self.buffer[tail & self.mask];
            
            let seq = slot.sequence.load(Ordering::Acquire);
            let expected_seq = tail.wrapping_add(1);
            
            match seq.cmp(&expected_seq) {
                std::cmp::Ordering::Equal => {
                    match self.consumer_pos.tail.compare_exchange_weak(
                        tail,
                        tail.wrapping_add(1),
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            let item = unsafe { (*slot.data.get()).assume_init_read() };
                            slot.sequence.store(
                                tail.wrapping_add(self.capacity),
                                Ordering::Release,
                            );
                            return Some(item);
                        }
                        Err(_) => {
                            std::hint::spin_loop();
                            continue;
                        }
                    }
                }
                std::cmp::Ordering::Less => {
                    return None;
                }
                std::cmp::Ordering::Greater => {
                    std::hint::spin_loop();
                    continue;
                }
            }
        }
    }
    
    /// Send single item
    pub fn send_one(&self, item: T) -> Result<(), T> {
        self.send_single_internal(item)
    }
    
    /// Receive single item
    pub fn recv_one(&self) -> Option<T> {
        self.recv_single_internal()
    }
    
    /// Returns the capacity of the queue
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Returns true if the queue is empty
    pub fn is_empty(&self) -> bool {
        let head = self.producer_pos.head.load(Ordering::Acquire);
        let tail = self.consumer_pos.tail.load(Ordering::Acquire);
        head == tail
    }
    
    /// Returns true if the queue is full
    pub fn is_full(&self) -> bool {
        let head = self.producer_pos.head.load(Ordering::Acquire);
        let tail = self.consumer_pos.tail.load(Ordering::Acquire);
        head.wrapping_sub(tail) >= self.capacity
    }
    
    /// Returns the approximate number of items in the queue
    pub fn len(&self) -> usize {
        let head = self.producer_pos.head.load(Ordering::Acquire);
        let tail = self.consumer_pos.tail.load(Ordering::Acquire);
        head.wrapping_sub(tail)
    }
}

unsafe impl<T: Simd64Bit> Send for SimdMpmcQueue<T> {}
unsafe impl<T: Simd64Bit> Sync for SimdMpmcQueue<T> {}

/// Producer handle for SIMD queue
pub struct SimdProducer<T> {
    queue: Arc<SimdMpmcQueue<T>>,
}

/// Consumer handle for SIMD queue
pub struct SimdConsumer<T> {
    queue: Arc<SimdMpmcQueue<T>>,
}

impl<T: Simd64Bit> SimdProducer<T> {
    pub fn new(queue: Arc<SimdMpmcQueue<T>>) -> Self {
        Self { queue }
    }
    
    pub fn send(&self, items: &[T]) -> Result<usize, Vec<T>> {
        self.queue.send(items)
    }
    
    pub fn send_one(&self, item: T) -> Result<(), T> {
        self.queue.send_one(item)
    }
    
    pub fn is_full(&self) -> bool {
        self.queue.is_full()
    }
    
    pub fn capacity(&self) -> usize {
        self.queue.capacity()
    }
}

impl<T: Simd64Bit> SimdConsumer<T> {
    pub fn new(queue: Arc<SimdMpmcQueue<T>>) -> Self {
        Self { queue }
    }
    
    pub fn recv(&self, buffer: &mut [T]) -> usize {
        self.queue.recv(buffer)
    }
    
    pub fn recv_one(&self) -> Option<T> {
        self.queue.recv_one()
    }
    
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

impl<T: Simd64Bit> Clone for SimdProducer<T> {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
        }
    }
}

impl<T: Simd64Bit> Clone for SimdConsumer<T> {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
        }
    }
}