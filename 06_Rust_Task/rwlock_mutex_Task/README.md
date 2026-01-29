#  Rust Server Request Tracker

A simple Rust project that demonstrates how to build a **server-style request tracker** using:

- `enum` for request modeling
- `static` variables
- `lazy_static` for global initialization
- `ref` for the reference
- `Mutex` and `RwLock` for thread-safe shared state
- Pattern matching with `match`

---

##  Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Concepts Used (Brief Explanation)](#concepts-used-brief-explanation)
- [Request Model](#request-model)
- [Code Walkthrough](#code-walkthrough)
- [Running the Program](#running-the-program)
- [Sample Output](#sample-output)
- [Conclusion](#conclusion)
- [Resources](#resources)

---

##  Introduction

This project simulates a **basic server request handler** that processes different types of requests such as `GET`, `POST`, and `DELETE`.

It keeps track of:
- The **total number of requests processed**
- The **total number of request per count**

The project highlights how Rust ensures **memory safety and thread safety** using ownership and synchronization primitives.

---

##  Features

- Strongly typed request handling using `enum`
- Global request counter using `Mutex`
- Shared read/write data using `RwLock`
- Exhaustive pattern matching
- Clean and extensible design

---

##  Concepts Used (Brief Explanation)

### 1. `enum`
Used to define different request types in a **type-safe way**.

- Prevents invalid states
- Forces exhaustive handling via `match`

Example:
```rust
enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),

}
``` 

### 2. `lazy_static!`
`lazy_static` allows defining **global variables that require runtime initialization**.

Why it is needed:
- `Mutex<T>` cannot be directly initialized in a `static`
- `lazy_static` ensures initialization happens **once**, safely

Example:
```rust
lazy_static! {
    static ref REQUEST_COUNT: Mutex<u32> = Mutex::new(0);
}
```

### 3. `Mutex<T>`

- A `Mutex` (Mutual Exclusion) ensures that **only one thread can access** the data at a time.
- Used here to:
    - Safely increment counters
    - Prevent race conditions
    - Each request type has its own Mutex<u32> counter.

Example:
```rust
lazy_static! {
    static ref REQUEST_COUNT: Mutex<u32> = Mutex::new(0);
}

fn handle_request(req: Request) -> String {
    {
        let mut count = REQUEST_COUNT.lock().unwrap();
        *count += 1;
    }
}
```

### 4. `RwLock<T>`

- this type of lock allows **a number of readers** or at **most one writer at any point in time**. The write portion of this lock typically allows  modification of  the underlying data (exclusive access) and the read portion of this lock typically allows for read-only access (shared access).

- In comparison, a **Mutex** does not **distinguish between readers or writers** that acquire the lock, therefore blocking any threads waiting for the lock to become available. An `RwLock` will **allow any number of readers** to acquire the lock as long as a writer is not holding the lock.

Example 

```Rust


lazy_static! {
    static ref REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
    static ref GET_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
    static ref POST_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
    static ref DELETE_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
}
```

## Code Walkthrough

```Rust
fn handle_request(req: Request) -> String {
    {
        let mut count = REQUEST_COUNT.write().unwrap();
        *count += 1;
    }

    match req {
        Request::Get { endpoint } => {
            {
                let mut get_count = GET_REQUEST_COUNT.write().unwrap();
                *get_count += 1;
            }

            format!("Handling GET request to {}", endpoint)
        }
        Request::Post {
            endpoint,
            payload_size,
        } => {
            {
                let mut post_count = POST_REQUEST_COUNT.write().unwrap();
                *post_count += 1;
            }

            format!(
                "Handling POST request to {} with payload size {}",
                endpoint, payload_size
            )
        }
        Request::Delete(id) => {
            {
                let mut delete_count = DELETE_REQUEST_COUNT.write().unwrap();
                *delete_count += 1;
            }

            format!("Handling DELETE request for id {}", id)
        }
    }
}

```

## Request Model

Requests are represented using an `enum`:

```rust
enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),
}
``` 

## Running the Program

cd request_tracker_using_mutex

cargo.toml
```Rust
[dependencies]
lazy_static = "1.5"
```
> cargo run

## Sample Output

```
shivam@shivam-laptop:~/Desktop/inventyv_training/06_Rust_Task/rwlock_mutex_Task/request_tracker_using_rwlock$ cargo run
   Compiling request_tracker_using_rwlock v0.1.0 (/home/shivam/Desktop/inventyv_training/06_Rust_Task/rwlock_mutex_Task/request_tracker_using_rwlock)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/request_tracker_using_rwlock`
Handling GET request to /api/data
Handling POST request to /api/upload with payload size 1024
Handling DELETE request for id 1010
Total requests processed: 3
```

## Conclusion

- This project demonstrates how Rust enables safe and efficient concurrency using:
- Strong type system (enum)
- Safe global state (static + Mutex/RwLock)
- Exhaustive control flow (match)
- It is a great foundation for understanding real-world server-side Rust development and prepares you for interview questions on concurrency and synchronization.


##  Resources 

- [Rust Book Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Mutex Documentation](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [Rust RwLock Documentation](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
- [Rust Pattern Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html)