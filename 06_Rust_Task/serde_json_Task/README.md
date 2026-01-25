# Serde JSON

## Table of Contents

- [Introduction](#introduction)
- [Serializing JSON](#serializing-json)
- [Deserializing JSON](#deserializing-json)

## Introduction

Serde JSON is a powerful and easy-to-use JSON serialization and deserialization library for Rust. It provides a simple and efficient way to convert Rust data structures into JSON and vice versa.

By using Serde JSON, you can easily serialize and deserialize JSON data in your Rust programs. This can be useful for a variety of purposes, such as sending data over the network, storing data in a database, or sharing data between different parts of your application.

In this tutorial, we will cover the basics of using Serde JSON to serialize and deserialize JSON data in Rust. We will start by installing Serde JSON and then learn how to serialize and deserialize JSON data using Serde JSON.



## Serializing JSON

To serialize a Rust data structure to JSON, use the `serde_json::to_string()` function. This function takes a reference to the data structure as an argument and returns a `Result` containing the serialized JSON string on success or an error on failure.

Here's an example of how to serialize a `User` struct to JSON:

```rust
use serde_json;

struct User {
    id: u64,
    username: String,
    profile: UserProfile,
    location: UserLocation,
    roles: UserRoles,
    presence: UserPresence,
}

n main() {
    let  user = User {
        id: 1,
        username: "shivam".to_string(),
        profile: UserProfile {
            display_name: "shivamsinh".to_string(),
        },
        location: UserLocation {
            country_code: "IN".to_string(),
            timezone: "Asia/Kolkata".to_string(),
        },
        roles: UserRoles {
            roles: vec!["user".to_string()],
        },

        presence: UserPresence {
            status: "online".to_string(),
        },
    };
    
    println!("{:#?}", user);

    // conver struct -> JSON (Serializetion)

    let json_user = serde_json::to_string_pretty(&user).unwrap();
    println!("{}",json_user)


}
```

In this example, we create an instance of the `User` struct and serialize it to a JSON string using `serde_json::to_string()`. The `unwrap()` method is used to handle any errors that may occur during serialization.

## Deserializing JSON

To deserialize JSON to a Rust data structure, use the `serde_json::from_str()` function. This function takes a string slice containing the JSON data as an argument and returns a `Result` containing the deserialized data structure on success or an error on failure.

Here's an example of how to deserialize a JSON string to a `User` struct:

```rust
use serde_json;

struct User {
    id: u64,
    username: String,
    profile: UserProfile,
    location: UserLocation,
    roles: UserRoles,
    presence: UserPresence,
}
fn main() {
    let user = User {
        id: 1,
        username: "shivam".to_string(),
        profile: UserProfile {
            display_name: "shivamsinh".to_string(),
        },
        location: UserLocation {
            country_code: "IN".to_string(),
            timezone: "Asia/Kolkata".to_string(),
        },
        roles: UserRoles {
            roles: vec!["user".to_string()],
        },

        presence: UserPresence {
            status: "online".to_string(),
        },
    };
    // conver struct -> JSON (Serializetion)
    let json_user = serde_json::to_string_pretty(&user).unwrap();
    // println!("{}", json_user);

    // Deserializetion (JSON -> Object (struct))

    // 1) method

    let user1: User = serde_json::from_str(&json_user).unwrap();

    println!("{:#?}", user1);

    // 2) method

    let user_data = r#"{
                    "id": 2,
                    "username": "shivam@123",
                    "profile": {
                        "display_name": "shivamsinh Gohil"
                    },
                    "location": {
                        "country_code": "IN",
                        "timezone": "Asia/Kolkata"
                    },
                    "roles": {
                        "roles": [
                        "user"
                        ]
                    },
                    "presence": {
                        "status": "offline"
                    }
                }"#;

    let user2:User = serde_json::from_str(user_data).unwrap();

    println!(" user2 = {:#?}",user2);


}

```

In this example, we create a JSON string containing the serialized `User` struct and deserialize it to a `User` struct using `serde_json::from_str()`. The `unwrap()` method is used to handle any errors that may occur during deserialization.

## Conclusion

Serde JSON is a powerful and easy-to-use JSON serialization and deserialization library for Rust. It provides a simple and efficient way to convert Rust data structures into JSON and vice versa. By using Serde JSON, you can easily serialize and deserialize JSON data in your Rust programs.

## Resources

- [Serde JSON Documentation](https://docs.rs/serde_json/latest/serde_json/)
- [Serde JSON GitHub Repository](https://github.com/serde-rs/json)

