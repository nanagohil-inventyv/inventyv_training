# Generic Inventory System in Rust

A simple Rust project that demonstrates how to build a **generic inventory management system** using:

* `generics` and `trait bounds`
* `HashMap` for efficient storage
* Custom `enum` for domain-specific error handling
* Trait-based polymorphism
* Clean, extensible, and reusable design

---

##  Table of Contents

* [Introduction](#introduction)
* [Features](#features)
* [Concepts Used (Brief Explanation)](#concepts-used-brief-explanation)
* [Item Display Trait](#item-display-trait)
* [Inventory Model](#inventory-model)
* [Error Handling](#error-handling)
* [Code Walkthrough](#code-walkthrough)
* [Running the Program](#running-the-program)
* [Sample Output](#sample-output)
* [Conclusion](#conclusion)
* [Resources](#resources)

---

##  Introduction

This project implements a **generic inventory system** that can store and manage different kinds of items.

The design focuses on:

* **Reusability** using generics
* **Extensibility** using traits
* **Safety** using Rust’s ownership and error handling

The inventory:

* Stores items by a unique ID
* Prevents duplicate or invalid IDs
* Allows safe retrieval of items
* Displays all items using a common interface

---

##  Features

* Generic `Inventory<T>` container
* Trait-based item display (`DisplayItem`)
* HashMap-based fast lookup
* Custom error handling using `Result`
* Clean separation of responsibilities
* Interview-ready Rust design

---

##  Concepts Used (Brief Explanation)

### 1 Generics (`Inventory<T>`)

Generics allow the inventory to store **any type**, not just a single concrete struct.

```rust
pub struct Inventory<T>
where
    T: DisplayItem + Clone,
```

This ensures:

* Type safety
* Code reuse
* Zero runtime overhead

---

### 2 Traits (`DisplayItem`)

The `DisplayItem` trait defines **behavior**, not data.

```rust
pub trait DisplayItem {
    fn display(&self) -> String;
}
```

Any type that wants to be stored in the inventory **must implement this trait**.

This allows:

* Polymorphism without inheritance
* Inventory logic to stay generic

---

### 3 HashMap for Storage

```rust
HashMap<String, T>
```

Why `HashMap`?

* Fast O(1) average lookup
* Enforces unique IDs
* Natural fit for ID → Item mapping

---

### 4 Clone Trait Bound

```rust
T: Clone
```

Required because:

* Inventory returns owned values
* Rust does not allow moving out of borrowed references
* `Clone` enables safe copying

---

## Item Display Trait

Example item type:

```rust
#[derive(Debug, Clone)]
pub struct Product {
    name: String,
    price: f64,
}

impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("Product : {} , Price ₹ {:.2}", self.name, self.price)
    }
}
```

This allows the inventory to display items **without knowing their internal structure**.

---

##  Inventory Model

```rust
pub struct Inventory<T>
where
    T: DisplayItem + Clone,
{
    items: HashMap<String, T>,
}
```

Responsibilities:

* Store items
* Validate IDs
* Enforce uniqueness
* Delegate display logic to items

---

##  Error Handling

Custom error enum:

```rust
pub enum InventoryError {
    Duplicatid(String),
    InvalidId,
    ItemNotFound(String),
}
```

Why custom errors?

* Clear domain-specific failures
* Avoid panics
* Allow graceful recovery

---

##  Code Walkthrough

###  Add Item

```rust
 pub fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }
        if self.items.contains_key(&id) {
            return Err(InventoryError::Duplicatid(id));
        }

        self.items.insert(id, item);

        Ok(())
    }
```

Checks:

* ID must not be empty
* ID must be unique

---

### Get Item

```rust
pub fn get_item(&self, id: String) -> Result<T, InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }
        self.items
            .get(&id)
            .cloned()
            .ok_or_else(|| InventoryError::ItemNotFound(id))
    }

```

Behavior:

* Returns cloned item
* Errors if ID is invalid or missing

---

###  Display All Items

```rust
pub fn display_all(&self) -> String
```

Behavior:

* Iterates over inventory
* Uses `DisplayItem::display()`
* Formats output consistently

---

##  Running the Program

```bash
cargo run
```

No external dependencies required.

---

##  Sample Output

```text
Error: Item with id P01 already exists
Error: Item with id P03 not found
Product { name: "Laptop", price: 60000.0 }
Erro: Invalid item Id
ID: P01
Product : Laptop , Price ₹ 60000.00
-------------------
ID: P02
Product : KeyBoard , Price ₹ 1000.00
-------------------
```

---

##  Conclusion

This project demonstrates:

* How to design **generic and reusable Rust APIs**
* How traits decouple behavior from storage
* How `Result` and custom errors improve robustness
* Real-world Rust patterns used in libraries and backends

It is an excellent foundation for:

* Inventory systems
* Resource managers
* Service registries
* Interview preparation

---

##  Resources

* Rust Book – Generics: [https://doc.rust-lang.org/book/ch10-00-generics.html](https://doc.rust-lang.org/book/ch10-00-generics.html)
* Rust Book – Traits: [https://doc.rust-lang.org/book/ch10-02-traits.html](https://doc.rust-lang.org/book/ch10-02-traits.html)
* Rust HashMap Docs: [https://doc.rust-lang.org/std/collections/struct.HashMap.html](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
* Rust Error Handling: [https://doc.rust-lang.org/book/ch09-00-error-handling.html](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
