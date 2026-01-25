## Table of Contents

- [Introduction](#introduction)
- [What is `#[derive(Debug)]`](#what-is-derivdebug)
- [Example](#example)

## Introduction

Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: You don’t have to rely on the order of the data to specify or access the values of an instance.

## what-is-derivdebug

`#[derive(Debug)]` is a **procedural macro attribute** in Rust that instructs the Rust compiler to **automatically generate** an implementation of the `std::fmt::Debug` trait for your custom type (struct, enum, or union).

### Why Do We Need It?

The `Debug` trait allows you to print values in a developer-friendly way using:

```Rust
println!("{:?}", my_value);     // Basic debug format
println!("{:#?}", my_value);   // Pretty-printed debug format
dbg!(&my_value);                // Macro that prints and returns the value

``` 

## Example

```Rust
#[derive(Debug)]
struct User {
    id: u64,
    username: String,
    profile: UserProfile,
    location: UserLocation,
    roles: UserRoles,
    presence: UserPresence,
}
```