//! # Response Module
//!
//! Provides utilities for generating HTTP responses, including:
//! - Formatting response content with headers
//! - Generating error responses
//! - Serving HTML content
//! - Handling redirects
//!
//! ## Example
//!
//! ```rust
//! use response::{format_content, html};
//!
//! // Format a response with session
//! let response = format_content(11, "Hello World", 12345);
//!
//! // Serve HTML content
//! let html_content = html("templates/index.html");
//! ```
//!
use std::usize;

/// Formats content into a complete HTTP response with headers.
///
/// Creates a response including:
/// - 200 OK status
/// - Session cookie
/// - Content-Length header
/// - The actual content
///
/// # Arguments
///
/// * `length` - Content length in bytes
/// * `content` - The response body content
/// * `session_id` - Session identifier to include in cookie
///
/// # Returns
///
/// A vector of bytes containing the complete formatted HTTP response
///
/// # Example
///
/// ```rust
/// let response = format_content(
///     11,
///     "Hello World",
///     12345
/// );
/// ```
pub fn format_content(length: usize, content: &str, session_id: usize) -> Vec<u8> {
    format!("HTTP/1.1 200 OK\r\nSet-Cookie: session_id={session_id}; HttpOnly\r\nContent-Lenght: {length}\r\n\r\n{content}").into_bytes()
}

// Generates a 404 Not Found error response.
///
/// # Returns
///
/// A string containing a complete 404 HTTP response
///
/// # Example
///
/// ```rust
/// let not_found = error();
/// assert!(not_found.contains("404 NOT FOUND"));
/// ```
pub fn error404() -> String {
    "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 21\r\n\r\nResource not found".into()
}

/// Reads and returns the contents of an HTML file.
///
/// # Arguments
///
/// * `path` - Path to the HTML file
/// **OBS:** The path must be relative to the root of your project
///
/// # Returns
///
/// The contents of the HTML file as a string
///
/// # Panics
///
/// Panics if:
/// - The file doesn't exist
/// - The file can't be read
/// - The file contains invalid UTF-8
///
/// # Example
///
/// ```rust
/// let content = html("templates/index.html");
/// ```
pub fn html(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(file_str) => file_str,
        Err(e) => panic!("{e}"),
    }
}

/// Handles request redirection by generating an 302 HTTP response to the new route.
///
/// # Arguments
///
/// * `route` - the destination route
///
/// # Returns
///
/// The response string from the redirect handler
///
pub fn redirect(route: &str) -> String {
    format!("HTTP/1.1 302 Found\r\nLocation: {route}\r\n\r\n")
}
