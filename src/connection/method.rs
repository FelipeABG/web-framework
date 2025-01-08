//! # Method Module
//!
//! Defines HTTP methods supported by the server.
//!
//! Currently supports GET and POST methods with string conversion
//! functionality in both directions.
//!
//! ## Example
//!
//! ```rust
//! use method::Method;
//!
//! // Create from string
//! let method = Method::from_string("GET");
//!
//! // Convert to string
//! assert_eq!(method.to_string(), "GET");
//! ```

use core::panic;

/// Represents HTTP methods supported by the server.
///
/// Currently supports:
/// - GET
/// - POST
#[derive(Debug)]
pub enum Method {
    /// HTTP GET method
    GET,
    /// HTTP POST method
    POST,
}

impl Method {
    /// Converts a string to a Method enum variant.
    ///
    /// # Arguments
    ///
    /// * `string` - The HTTP method as a string ("GET" or "POST")
    ///
    /// # Returns
    ///
    /// The corresponding Method enum variant
    ///
    /// # Panics
    ///
    /// Panics if the string is not "GET" or "POST"
    ///
    /// # Example
    ///
    /// ```rust
    /// let method = Method::from_string("GET");
    /// assert!(matches!(method, Method::GET));
    /// ```
    pub fn from_string(string: &str) -> Self {
        match string {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => panic!("Failed to parse string to http method"),
        }
    }

    /// Converts the Method enum variant to its string representation.
    ///
    /// # Returns
    ///
    /// A String containing "GET" or "POST"
    ///
    /// # Example
    ///
    /// ```rust
    /// let method = Method::GET;
    /// assert_eq!(method.to_string(), "GET");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            Method::GET => "GET".to_string(),
            Method::POST => "POST".to_string(),
        }
    }
}
