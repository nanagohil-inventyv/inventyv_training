# Session Manager – HashMap Method Demonstration (Rust)


##  Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Concepts Used (Brief Explanation)](#concepts-used-brief-explanation)
- [Code Walkthrough](#code-walkthrough)
- [Conclusion](#conclusion)
- [Resources](#resources)

---

## Introduction 

This project demonstrates a **real-world Rust data structure** (`SessionManager`) that uses a `HashMap` to manage user sessions.

The goal is to **clearly show practical usage of selected `HashMap` methods**, focusing on ownership, memory safety, and performance patterns used in backend systems.

##  Features

The `SessionManager` simulates a backend service that:

- Stores user sessions
- Consumes one-time tokens (OTP / auth tokens)
- Cleans up inactive sessions
- Merges session data from external sources
- Takes snapshots for rollback or auditing

### HashMap Methods Demonstrated

Only the following **five methods** are intentionally used:

| Method          | Purpose                                 |
|-----------------|-----------------------------------------|
| `try_reserve()` | Pre-allocate capacity safely            |
| `extend()`      | Bulk insert / merge session data        |
| `clone()`       | Create a snapshot for backup or rollback|
| `take()`        | Move a one-time value out safely        |
| `retain()`      | Remove expired or invalid sessions      |
 

## Concepts Used (Brief Explanation)

### `try_reserve()`
Used during initialization to avoid repeated rehashing and handle allocation failure safely.

### `extend()`
Used to load sessions in bulk from a database, config, or external source.

### `clone()`
Used to create a deep copy of the session state for auditing or rollback.

### `take()`
Used to consume a one-time token safely without removing the entire session.

### `retain()`
Used to clean up inactive or expired sessions in place.

## Code Walkthrough

```rust
#[derive(Debug, Clone)]
struct Session {
    token: Option<String>, // one-time consumable token for authentication
    last_active: SystemTime,
}

#[derive(Debug)]
struct SessionManager {
    sessions: HashMap<u64, Session>, // user_id -> Session
}

impl SessionManager {
    /// create manager with capacity for n users
    fn new(expected_users: usize) -> Self {
        let mut sessions = HashMap::new();
        sessions
            .try_reserve(expected_users)
            .expect("Failed to reserve capacity for sessions");
        Self { sessions }
    }

    /// Add session in bulk (e.g. from DB or config)

    fn load_sessions(&mut self, data: Vec<(u64, Session)>) {
        // extent -> merge sessions
        self.sessions.extend(data);
    }

    /// Take a snapshot (rollback)
    fn snapshot(&self) -> HashMap<u64, Session> {
        self.sessions.clone()
    }

    /// cleanup inactive sessions

    fn cleanup_inactive(&mut self, max_idle: Duration) {
        let now = SystemTime::now();

        self.sessions.retain(|_, session| {
            now.duration_since(session.last_active)
                .map(|d| d <= max_idle)
                .unwrap_or(false)
        });
    }

    /// Consume a one-time token for a user

    fn consume_token(&mut self, user_id: u64) -> Option<String> {
        if let Some(session) = self.sessions.get_mut(&user_id) {
            session.token.take()
        } else {
            None
        }
    }
}
```

## Conclusion 
This example demonstrates how` Rust’s HashMap `can be used safely, efficiently, and idiomatically in a real-world scenario. By focusing on a limited set of core methods `try_reserve`, `extend`, `clone`, `take`, and `retain`.

## Resources

- [HashMap Doc](https://doc.rust-lang.org/std/collections/struct.HashMap.html)