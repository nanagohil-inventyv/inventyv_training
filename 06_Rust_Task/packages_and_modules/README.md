#  packages_and_modules_task

##  Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Project Structure](#project-structure)
- [Architecture Overview](#architecture-overview)
- [Concepts Used (Brief Explanation)](#concepts-used-brief-explanation)
- [Code Walkthrough](#code-walkthrough)
- [Running the Program](#running-the-program)
- [Conclusion](#conclusion)
- [Resources](#resources)

---

##  Introduction

This project combines multiple Rust practice tasks into **a single Rust package** using Rust’s **module system (`mod`)**.

Each task is implemented as a module and executed from a single `main.rs`.


---

##  Features

- `loops_task.rs` → basic loop examples
- `struct_and_method.rs` → structs and `impl` blocks
- `serde_json_task`
  - `serialize.rs` → JSON serialization
  - `deserialize.rs` → JSON deserialization
- `concurrency_task`
  - `mutex_tracker.rs` → shared state with `Mutex`
  - `rwlock_tracker.rs` → shared state with `RwLoc
---

## Project Structure

```
06_rust_tasks/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── loops_task.rs
    ├── struct_and_method.rs
    ├── serde_json_task/
    │   ├── mod.rs
    │   ├── serialize.rs
    │   └── deserialize.rs
    ├── concurrency_task/
    │   ├── mod.rs
    │   ├── mutex_tracker.rs
    │   └── rwlock_tracker.rs
``` 

##  Architecture Overview

- **One Cargo package**
- **One crate** with `main.rs` as the entry point
- Tasks are organized using **modules and submodules**
- Folder-based modules use `mod.rs`
- Flat modules use `.rs` files at `src/` level



##  Concepts Used (Brief Explanation)

- Rust package & crate structure

- Module system (mod, mod.rs)

- Structs and method implementations

- JSON handling with serde and serde_json

- Concurrency using Mutex and RwLock

## Code Walkthrough

```rust
mod loops_task;
mod rwlock_mutex_task;
mod serde_json_task;
mod struct_and_method;

fn main() {
    loops_task::run();
    struct_and_method::run();
    serde_json_task::user_serialize::run();
    serde_json_task::user_deserialize::run();
    rwlock_mutex_task::request_tracker_using_mutex::run();
    rwlock_mutex_task::request_tracker_using_rwlock::run();
}
```

## Running the Program

``` bash
cargo run
``` 

## conclusion

- Learning Rust module organization
- Practicing real-world Rust project structure


## Resources

- [Rust Module](https://doc.rust-lang.org/rust-by-example/mod.html)
- [Rust Book](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)