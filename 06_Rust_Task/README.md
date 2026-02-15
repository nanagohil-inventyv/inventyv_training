# Rust Task 

##  Task Overview

###  01_loops_task
This task focuses on Rust loop constructs and control flow.

#### Concepts Covered:
- `loop`
- `while`
- `for`
- `break`
- `continue`
- Labeled `break` and `continue`
- Nested loops
- Bitwise XOR logic
- Subarray problem-solving

#### Example Problem:
> Print the first contiguous subarray whose bitwise XOR of only positive numbers is zero.

---

###  02_struct_and_method_impl

This task focuses on Rust structs and implementations.

#### Concepts Covered:
- `struct` definition
- `impl` blocks
- Methods and associated functions


#### Example Problem:
> using the `User` Entity  demonstrat use of getter and setter


###  03_serde_json_Task
 
This taks focuse on Rust Serde json 

#### Concepts Covered:

- Installing Ther serde json

```Rust
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.128"

```
- `Serialization` and `Deserialization` 

#### Example Problem:

> using `User` entity do the `Serialiazation` and `Deserialization`


### 04_rwlock_mutex_Task

- simple Rust project that demonstrates how to build a **server-style request tracker** using:
    - `enum` for request modeling
    - `lazy_static` for global initialization
    - `ref` for the reference
    - `Mutex` and `RwLock` for thread-safe shared state
    - Pattern matching with `match`

#### Concepts Covered:

- `static variables`
- `Mutex`
- `RwLock`
- `ref`
- `enum`
- `lazy_static!`


### 05_packages_and_modules_task

this task demonstrates that how we organize our big project into multiple `packages` , `modules` and `crates`.

#### Concepts Covered:
- `Packages`
- `modules`
- `crates`


### 06_hashmap_taks

This project demonstrates a **real-world Rust data structure** (`SessionManager`) that uses a `HashMap` to manage user sessions.

#### Concepts Covered:

| Method          | Purpose                                 |
|-----------------|-----------------------------------------|
| `try_reserve()` | Pre-allocate capacity safely            |
| `extend()`      | Bulk insert / merge session data        |
| `clone()`       | Create a snapshot for backup or rollback|
| `take()`        | Move a one-time value out safely        |
| `retain()`      | Remove expired or invalid sessions      |
 

### 07_ownership_of_structure

this project demonstrates a **ownership and borrowing**  consept using  `structure` type

#### Concepts Covered:
- `ownership rules`
- `borrowing rules`
- `mutable borrow`
- `immutable borrow`


### 08_inventory_management 


A simple Rust project that demonstrates how to build a **generic inventory management system** using:

* `generics` and `trait bounds`
* `HashMap` for efficient storage
* Custom `enum` for domain-specific error handling
* Trait-based polymorphism


#### Concepts Covered:
- `generics` and `trait bounds`
- `collections HashMap vector etc` 
- `custom error handling`


### 09_invetory_management_with_closure_and_lifetime

This project implements a **generic inventory system** that can store and manage different kinds of items with the use of `closures` and `lifetimes`.

#### Concepts Covered:

- `generics` and `trait bounds`
- `collections HashMap vector etc` 
- `custom error handling`
- `closures` and `lifetimes`


### 10_concurrency_and_thread_task

A simple Rust project that demonstrates how to build a **concurrent in-memory record system** using:

#### Concepts Covered: 

- `Arc` and `RwLock` for safe shared state
- `AtomicI32` for lock-free unique ID generation
- `chrono` for time-based record expiration
- `thread::spawn`

### 11_http_server

A simple Rust project that demonstrates how to build a **concurrent multi-threaded REST API server** using:

* `Arc` and `tokio::sync::RwLock` for safe shared state
* `article.json` for efficient in-memory storage
* `UUID` for unique ID generation
* `Tokio` multi-thread runtime for concurrent request handling
* Background async tasks for non-blocking JSON persistence
