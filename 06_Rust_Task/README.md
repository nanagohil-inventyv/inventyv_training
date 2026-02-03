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

#### Concepts Convered:

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

#### Concepts Convered:

- `static variables`
- `Mutex`
- `RwLock`
- `ref`
- `enum`
- `lazy_static!`


### 05_packages_and_modules_task

this task demonstrates that how we organize our big project into multiple `packages` , `modules` and `crates`.

#### Concepts Convered:
- `Packages`
- `modules`
- `crates`


### 06_hashmap_taks

This project demonstrates a **real-world Rust data structure** (`SessionManager`) that uses a `HashMap` to manage user sessions.

#### Concepts Convered:

| Method          | Purpose                                 |
|-----------------|-----------------------------------------|
| `try_reserve()` | Pre-allocate capacity safely            |
| `extend()`      | Bulk insert / merge session data        |
| `clone()`       | Create a snapshot for backup or rollback|
| `take()`        | Move a one-time value out safely        |
| `retain()`      | Remove expired or invalid sessions      |
 

### 07_ownership_of_structure

this project demonstrates a **ownership and borrowing**  consept using  `structure` type

#### Concepts Convered:
- `ownership rules`
- `borrowing rules`
- `mutable borrow`
- `immutable borrow`