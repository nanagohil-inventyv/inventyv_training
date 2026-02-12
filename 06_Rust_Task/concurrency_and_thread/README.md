# Multi-Threaded Record Management System in Rust

A simple Rust project that demonstrates how to build a **concurrent in-memory record system** using:

* `Arc` and `RwLock` for safe shared state
* `AtomicI32` for lock-free unique ID generation
* `chrono` for time-based record expiration
* Background worker threads for cleanup and metrics

---

## Table of Contents

* [Introduction](#introduction)
* [Features](#features)
* [Concepts Used (Brief Explanation)](#concepts-used-brief-explanation)
* [Data Model](#data-model)
* [Thread Responsibilities](#thread-responsibilities)
* [Code Walkthrough](#code-walkthrough)
* [Running the Program](#running-the-program)
* [Sample Output](#sample-output)
* [Conclusion](#conclusion)
* [Resources](#resources)

---

## Introduction

This project demonstrates how to build a **multi-threaded shared memory system in Rust**.

The system:

- Periodically creates new records
- Maintains them in shared memory
- Automatically removes expired records (TTL-based cleanup)
- Tracks even and odd record counts
- Continuously prints system state


---

## Features

- Automatic record creation every 10 seconds
- TTL-based cleanup (records older than 20 seconds are removed)
- Even and odd record filtering
- Continuous state printing
- Atomic ID generation without locks
- Thread-safe shared memory using `RwLock`

---

## Concepts Used (Brief Explanation)

### 1 Arc (Atomic Reference Counting)

Allows multiple threads to safely share ownership of the same data.

```rust
Arc<RwLock<Vec<MultiThread>>>
```

---

### 2 RwLock

- Multiple readers allowed simultaneously
- Only one writer allowed at a time
- Efficient for read-heavy workloads

Used for:
- Printing records
- Counting records
- Cleaning expired records
- Adding new records

---

### 3 AtomicI32

Generates unique IDs without needing a mutex.

```rust
counter.fetch_add(1, Ordering::SeqCst)
```

Ensures no race conditions.

---

### 4 Chrono (Time Handling)

Used to:

- Store record creation time
- Convert string timestamps back to `DateTime`
- Calculate expiration differences

```rust
DateTime::parse_from_str(...)
```

---

### 5 retain()

Used to safely remove expired records:

```rust
vec.retain(|record| condition)
```

---

## Data Model

```rust
#[derive(Debug, Clone)]
struct MultiThread {
    id: i32,
    record_added_time: String,
    thread_id: String,
}
```

### Fields

- `id` → Unique primary key
- `record_added_time` → Timestamp stored as string
- `thread_id` → Random identifier simulating thread origin

---

## Thread Responsibilities

### 🟢 Thread 1 — Record Creator

- Runs every 10 seconds
- Generates a unique ID
- Inserts a new record into shared memory

---

### 🖨️ Thread 2 — State Printer

- Runs every 5 seconds
- Prints the entire in-memory record list

---

### 🧹 Thread 3 — Even Record Cleaner

Removes records that:

- Have an even ID
- Were added more than 20 seconds ago

---

### 🧹 Thread 4 — Odd Record Cleaner

Removes records that:

- Have an odd ID
- Were added more than 20 seconds ago

---

### 🔢 Thread 5 — Even Counter

- Counts total records with even IDs
- Prints result every 5 seconds

---

### 🔢 Thread 6 — Odd Counter

- Counts total records with odd IDs
- Prints result every 5 seconds

---

## Code Walkthrough

### Shared Store Initialization

```rust
let shared_store = Arc::new(RwLock::new(Vec::new()));
```

---

### Atomic ID Generation

```rust
let new_id = counter.fetch_add(1, Ordering::SeqCst) + 1;
```

---

### Record Creation

```rust
let record = MultiThread {
    id: new_id,
    record_added_time: Local::now().to_string(),
    thread_id: rand::random::<i32>().to_string(),
};
```

---

### Expired Record Cleanup

```rust
store.retain(|record| {
    let diff = now - convert_str_to_local_time(&record.record_added_time);
    diff.num_seconds() <= 20
});
```

---

### Counting Records

```rust
let even_count = store.iter().filter(|r| r.id % 2 == 0).count();
let odd_count = store.iter().filter(|r| r.id % 2 != 0).count();
```

---

## Running the Program

### 1️⃣ Add Dependencies

In `Cargo.toml`:

```toml
chrono = "0.4"
rand = "0.8"
```

---

### 2️⃣ Run the Project

```bash
cargo run
```

---

## Sample Output

```
Even Count: 2
Odd Count: 3

MultiThread {
    id: 1,
    record_added_time: "2026-02-12 11:17:36.921378616 +05:30",
    thread_id: "12345",
}

Even Count: 3
Odd Count: 4
```

Expired records automatically disappear after 20 seconds.

---

## Conclusion

This project demonstrates:

- Practical multi-threading in Rust
- Safe shared state management
- Background cleanup workers
- Atomic operations
- TTL-based record expiration

It resembles patterns used in:

- In-memory caches
- Job schedulers
- Distributed coordination systems
- Real-time monitoring services

---

## Resources
- [Rand Documentation](https://docs.rs/rand/latest/rand/)
- [Chrono Documentation](https://docs.rs/chrono/latest/chrono/)
- [Concurrency ](https://doc.rust-lang.org/book/ch16-01-threads.html)
---
