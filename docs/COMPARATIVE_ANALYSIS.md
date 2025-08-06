# Algorithmic Analysis: mpmc-std Design Principles

This document examines the algorithmic foundations of `mpmc-std` compared to established queue implementations, focusing on design trade-offs and architectural differences rather than performance claims.

## Algorithmic Approaches Overview

### Memory Layout Comparison

```
LMAX Disruptor (Ring Buffer):
┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
│  0  │  1  │  2  │  3  │  4  │  5  │  6  │  7  │
└─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
      ↑                               ↑
   Producer                       Consumer
   Sequence                       Sequence

Michael & Scott (Linked Nodes):
Head → [Node] → [Node] → [Node] → [Node] → Tail
       ┌─────┐   ┌─────┐   ┌─────┐   ┌─────┐
       │Data │   │Data │   │Data │   │Data │
       │Next*│   │Next*│   │Next*│   │Next*│
       └─────┘   └─────┘   └─────┘   └─────┘

mpmc-std (Sequence-Coordinated Array):
┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
│ S:0 │ S:1 │ S:2 │ S:3 │ S:4 │ S:5 │ S:6 │ S:7 │
│ [D] │ [D] │ [D] │ [D] │ [D] │ [D] │ [D] │ [D] │
└─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
S = Sequence Number, D = Data Slot
```

## Core Algorithmic Differences

### 1. LMAX Disruptor Algorithm

**Core Principle**: Ring buffer with sequence barriers and memory barriers for coordination.

```
Algorithm: Disruptor Send
1. claim_slot = next_sequence++
2. wait_for_slot_available(claim_slot)
3. write_data_to_slot(claim_slot)
4. publish_sequence(claim_slot)
5. notify_consumers()

Memory Barriers:
- Store-Store before publish
- Load-Acquire for dependency tracking
- Complex barrier coordination for multiple consumers
```

**Key Characteristics**:
- **Coordination**: Complex sequence barriers and dependency chains
- **Memory Management**: Pre-allocated ring buffer
- **Ordering**: Strong ordering guarantees with memory barriers
- **Complexity**: Multiple sequence types (producer, consumer, gating)

### 2. Michael & Scott Algorithm

**Core Principle**: Lock-free linked list with atomic compare-and-swap on head/tail pointers.

```
Algorithm: Michael & Scott Enqueue
1. node = allocate_new_node(data)
2. loop:
3.   tail = load_tail()
4.   next = load_tail_next() 
5.   if tail == current_tail:
6.     if next == null:
7.       if CAS(tail.next, null, node):
8.         break
9.     else:
10.      CAS(tail_ptr, tail, next)  // Help move tail
11. CAS(tail_ptr, tail, node)

Memory Pattern:
- Dynamic allocation per operation
- Pointer chasing through linked structure
- ABA problem mitigation required
```

**Key Characteristics**:
- **Coordination**: Atomic pointer manipulation with helping protocol
- **Memory Management**: Dynamic allocation per enqueue/dequeue
- **Ordering**: Natural FIFO through linked structure
- **Complexity**: ABA problem handling, memory reclamation

### 3. mpmc-std Algorithm

**Core Principle**: Array slots with per-slot sequence numbers for coordination.

```
Algorithm: mpmc-std Send
1. pos = head.fetch_add(1, Relaxed)
2. slot = &buffer[pos & mask]
3. while slot.sequence.load(Acquire) != pos:
4.   spin_loop_hint()
5. slot.item.write(data)
6. slot.sequence.store(pos + 1, Release)

Sequence States:
- slot.sequence == pos     → Ready for producer
- slot.sequence == pos + 1 → Data available for consumer  
- slot.sequence == pos + N → Consumed, waiting for wraparound
```

**Key Characteristics**:
- **Coordination**: Per-slot sequence matching
- **Memory Management**: Fixed pre-allocated array
- **Ordering**: Mathematical sequence progression
- **Complexity**: Simple slot state machine

## Detailed Algorithmic Analysis

### Coordination Mechanisms

#### Disruptor: Barrier-Based Coordination
```
Producer Barriers:     Consumer Barriers:
┌─────────────────┐   ┌─────────────────┐
│ Claim Sequence  │   │ Wait for Data   │
│ Wait for Space  │   │ Process Batch   │
│ Write Data      │   │ Update Cursor   │
│ Publish Batch   │   │ Signal Complete │
└─────────────────┘   └─────────────────┘
        │                       │
        └──── Memory Barriers ──┘
```

#### mpmc-std: Sequence Matching
```
Producer Flow:              Consumer Flow:
┌─────────────────┐        ┌─────────────────┐
│ Get Position    │        │ Get Position    │
│ Wait for Slot   │ ←---→  │ Wait for Data   │
│ Write Data      │        │ Read Data       │
│ Release Slot    │        │ Release Slot    │
└─────────────────┘        └─────────────────┘
```

## Comparison Summary Table

| Feature                | LMAX Disruptor         | Michael & Scott        | mpmc-std                  |
|------------------------|------------------------|------------------------|---------------------------|
| **Coordination**       | Sequence barriers      | Atomic pointer ops     | Per-slot sequence numbers |
| **Memory Layout**      | Ring buffer            | Linked nodes           | Fixed array               |
| **Memory Management**  | Pre-allocated          | Dynamic allocation     | Pre-allocated             |
| **Ordering**           | Strong (barriers)      | FIFO (linked list)     | Mathematical sequence     |
| **Complexity**         | High (multiple seqs)   | Medium (ABA, helping)  | Low (slot state machine)  |
| **Scalability**        | High (batch, barriers) | High (lock-free)       | High (lock-free, simple)  |
| **ABA Handling**       | Not required           | Required               | Not required              |
| **Producer/Consumer**  | Multiple               | Multiple               | Multiple                  |
| **Typical Use Case**   | Low-latency trading    | General-purpose queues | General-purpose queues    |

