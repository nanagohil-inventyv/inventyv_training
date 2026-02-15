# Multi-Threaded Article & Comment HTTP Server in Rust

A simple Rust project that demonstrates how to build a **concurrent multi-threaded REST API server** using:

* `Arc` and `tokio::sync::RwLock` for safe shared state
* `article.json` for efficient in-memory storage
* `UUID` for unique ID generation
* `Tokio` multi-thread runtime for concurrent request handling
* Background async tasks for non-blocking JSON persistence

---

## Table of Contents

* [Introduction](#introduction)
* [Features](#features)
* [Concepts Used (Brief Explanation)](#concepts-used-brief-explanation)
* [Data Model](#data-model)
* [Code Walkthrough](#code-walkthrough)
* [Running the Program](#running-the-program)
* [Sample Output](#sample-output)
* [Conclusion](#conclusion)
* [Resources](#resources)

---

## Introduction

This project demonstrates how to build a **multi-threaded HTTP server in Rust using Axum**.

The system:

- Handles multiple concurrent HTTP requests
- Maintains articles in shared memory
- Allows adding comments to articles
- Persists data into `articles.json`
- Uses Tokio’s multi-thread scheduler
- Ensures safe concurrent access using `RwLock`

This mimics real-world backend services used in production systems.

---

## Features

- Create, Read, Update, Delete Articles
- Add Comments to Articles
- Multi-threaded request handling
- Safe shared memory using `Arc<RwLock<_>>`
- UUID-based ID generation
- Background async JSON file saving
- RESTful API design


---

## Concepts Used (Brief Explanation)

### 1 Arc (Atomic Reference Counting)

Allows multiple threads to safely share ownership of the same data.

```rust
pub type SharedState = Arc<RwLock<Vec<Article>>>;
```

### 2 tokio::sync::RwLock

- Multiple readers allowed simultaneously
- Only one writer allowed at a time
- Async-aware (does not block runtime threads)

Used for:
- Reading articles
- Updating articles
- Deleting articles
- Adding comments


### 3 Tokio Multi-Thread Runtime

Configured using:
```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
```

### 4 UUID

Generates unique IDs for articles and comments:
```rust
Uuid::new_v4().to_string()
```

### 5 Async File Persistence

Uses:
```rust
tokio::fs::write(...)
```
File saving runs in background tasks:

```rust
tokio::spawn(...)
```
Prevents blocking request threads.


## Data Model 

```rust 
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Article {
    id: String,
    title: String,
    content: String,
    comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Comment {
    id: String,
    author: String,
    text: String,
}
```

Fields
- id → Unique identifier (UUID)
- title → Article title
- content → Article body
- comments → List of associated comments

## Code Walkthrough

### Shared Store Initialization
```rust
let articles = handler::load_articles();
let state = Arc::new(RwLock::new(articles));
```

### Article Creation
```rust
pub async fn create_article(
    State(state): State<SharedState>,
    Json(mut article): Json<Article>,
) -> (StatusCode, Json<Article>) {
    article.id = Uuid::new_v4().to_string();
    article.comments = vec![];

    let mut articles = state.write().await;
    articles.push(article.clone());

    save_articles(&articles);

    (StatusCode::CREATED, Json(article))
}
```

### Article Lookup

```rust
pub async fn get_article(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Article>, StatusCode> {
    let articles = state.read().await;

    articles
        .iter()
        .find(|a| a.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
```


### Adding Comment

```rust

pub async fn add_comment(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(mut comment): Json<Comment>,
) -> Result<(StatusCode, Json<Comment>), StatusCode> {
    comment.id = Uuid::new_v4().to_string();

    let mut articles = state.write().await;

    if let Some(article) = articles.iter_mut().find(|a| a.id == id) {
        article.comments.push(comment.clone());

        save_articles(&articles);

        Ok((StatusCode::CREATED, Json(comment)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
```

## Running the Program

### 1 Add Dependencies

> In Cargo.toml:

```rust
[dependencies]
axum = "0.8.1"
tokio = {version="1.44.1", features = ["full"]}
serde = {version = "1.0", features = ["derive"]}
uuid = {version = "1.16.0", features = ["v4"]}
serde_json = "1"
```

### 2 Run the Project 

> Server starts at:

```
Server running at http://127.0.0.1:8080
```


## Sample Output

Create Article

```
POST /articles
```

```json

{
    "id": "3f1a9c7e-6c54-4b0a-9a21-6a0c0d5f2a11",
    "title": "Getting Started with Rust",
    "content": "Rust is a systems programming language focused on safety and performance.",
    "comments": [
      {
        "id": "8b7d0f5a-3a52-4d6b-9f2e-1e2c3d4a5b6c",
        "author": "Shivam",
        "text": "Great introduction! Very helpful."
      },
      {
        "id": "2c9e1d7f-4b3a-4e91-a9f8-7b6c5d4e3f2a",
        "author": "Rutu",
        "text": "Can you also explain ownership in detail?"
      }
    ]
  }
```

### Add Comment

```base
POST /articles/{id}/comments
```
Response:
```json
{
    "id": "a1b2c3d4-e5f6-4789-9012-abcdef123456",
    "title": "Getting Started with Rust and basic of rust........",
    "content": "Rust is a systems programming language focused on safety and performance.",
    "comments": [
      {
        "id": "8b7d0f5a-3a52-4d6b-9f2e-1e2c3d4a5b6c",
        "author": "Shivam",
        "text": "Great introduction! Very helpful."
      },
      {
        "id": "2c9e1d7f-4b3a-4e91-a9f8-7b6c5d4e3f2a",
        "author": "Jagruti",
        "text": "Can you also explain ownership in detail?"
      }
    ]
}
```

## Conclusion

- This project demonstrates:
    - Practical multi-threaded backend development
    - Async programming with Tokio
    - Safe shared state management
    - REST API design with Axum
    - JSON-based persistence

## Resources
- [https://docs.rs/axum/latest/axum](https://docs.rs/axum/latest/axum/)
- [https://tokio.rs](https://tokio.rs/)
- [https://docs.rs/uuid/latest/uuid](https://docs.rs/uuid/latest/uuid/)