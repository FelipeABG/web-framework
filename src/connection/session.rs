//! # Session Management Module
//!
//! This module provides structures and functionality for managing sessions.
//! Sessions allow you to store and retrieve data of any type using unique keys.
//! Additionally, a `Sessions` manager is provided to manage multiple `Session` instances
//! with unique IDs.
//!
//! ## Overview
//!
//! - **Session**: A single session that stores key-value pairs, where the values can be
//!   of any type that implements the `Any` trait.
//! - **Sessions**: A manager for multiple sessions, allowing creation and retrieval
//!   of sessions by their unique IDs.
//!
//! ## Usage
//!
//! ### Example: Working with a Single Session
//! ```rust
//! use rwf::{Session, Sessions};
//!
//! let mut session = Session::new();
//! session.add("username".to_string(), "Alice");
//! session.add("age".to_string(), 30);
//!
//! let username: Option<&str> = session.get("username");
//! let age: Option<&i32> = session.get("age");
//!
//! assert_eq!(username, Some(&"Alice"));
//! assert_eq!(age, Some(&30));
//! ```
//!
//! ### Example: Managing Multiple Sessions
//! ```rust
//! use rwf::{Session, Sessions};
//!
//! let mut sessions = Sessions::new();
//!
//! // Add a new session and get its ID
//! let session_id = sessions.add();
//!
//! // Access the session by its ID
//! let session = sessions.get(session_id);
//! session.add("is_logged_in".to_string(), true);
//!
//! let is_logged_in: Option<&bool> = session.get("is_logged_in");
//! assert_eq!(is_logged_in, Some(&true));
//! ```
//!
//! ## Key Features
//!
//! - **Flexible Data Storage**: Store values of any type that implements `Any`.
//! - **Unique Session IDs**: Manage multiple sessions with unique identifiers.
//! - **Type Safety**: Retrieve stored values with type safety by specifying the expected type.

use std::{any::Any, collections::HashMap};

/// Represents a session that can store and retrieve data of any type.
///
/// The `Session` struct provides a flexible way to store and retrieve data using
/// string keys. Internally, it uses a `HashMap` to manage the data, where the values
/// are stored as `Box<dyn Any>`.
pub struct Session {
    /// The internal data storage for the session.
    data: HashMap<String, Box<dyn Any>>,
}

impl Session {
    /// Creates a new, empty `Session`.
    ///
    /// # Examples
    ///
    /// ```
    /// let session = Session::new();
    /// ```
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Adds a key-value pair to the session.
    ///
    /// The value can be of any type that implements the `Any` trait.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key for the value.
    /// * `value` - The value to store, which can be of any type.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut session = Session::new();
    /// session.add("username".to_string(), "Alice");
    /// ```
    pub fn add<T: Any>(&mut self, key: String, value: T) {
        self.data.insert(key, Box::new(value));
    }

    /// Retrieves a reference to the value associated with the given key, if it exists.
    ///
    /// The caller must specify the expected type of the value, and the method will return
    /// `None` if the type does not match or the key does not exist.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice representing the key to look up.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut session = Session::new();
    /// session.add("username".to_string(), "Alice");
    /// let username: Option<&str> = session.get("username");
    /// assert_eq!(username, Some(&"Alice"));
    /// ```
    pub fn get<T: Any>(&self, key: &str) -> Option<&T> {
        self.data.get(key)?.downcast_ref()
    }
}

/// Manages multiple sessions, each identified by a unique ID.
///
/// The `Sessions` struct provides functionality to create, store, and retrieve multiple
/// `Session` instances.
pub struct Sessions {
    /// A map of session IDs to their corresponding `Session` instances.
    sessions: HashMap<usize, Session>,
    /// A counter to generate unique session IDs.
    counter: usize,
}

impl Sessions {
    /// Creates a new, empty `Sessions` manager.
    ///
    /// # Examples
    ///
    /// ```
    /// let sessions = Sessions::new();
    /// ```
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            counter: 0,
        }
    }

    /// Checks whether a session with the given ID exists.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to the session ID to check.
    ///
    /// # Examples
    ///
    /// ```
    /// let sessions = Sessions::new();
    /// assert!(!sessions.contains(&1));
    /// ```
    pub fn contains(&self, id: &usize) -> bool {
        self.sessions.contains_key(id)
    }

    /// Adds a new session and returns its unique ID.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sessions = Sessions::new();
    /// let session_id = sessions.add();
    /// assert!(sessions.contains(&session_id));
    /// ```
    pub fn add(&mut self) -> usize {
        self.sessions.insert(self.counter, Session::new());
        let prev = self.counter;
        self.counter += 1;
        prev
    }

    /// Retrieves a mutable reference to the session with the given ID.
    ///
    /// # Arguments
    ///
    /// * `key` - The session ID to retrieve.
    ///
    /// # Panics
    ///
    /// Panics if the session ID does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sessions = Sessions::new();
    /// let session_id = sessions.add();
    /// let session = sessions.get(session_id);
    /// ```
    pub fn get(&mut self, key: usize) -> &mut Session {
        self.sessions.get_mut(&key).unwrap()
    }
}
