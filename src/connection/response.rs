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

/// A macro for loading and optionally processing template files with variable substitution.
///
/// # Usage
///
/// This macro provides two ways to work with template files:
///
/// 1. Simple file loading:
/// ```rust
/// let content = template!("path/to/file.txt");
/// ```
///
/// 2. Template processing with variable substitution:
/// ```rust
/// let name = "Alice";
/// let age = "25";
/// let content = template!("path/to/template.txt", name, age);
/// ```
///
/// # Arguments
///
/// * `$path` - A string literal representing the path to the template file
/// * `$value` - (Optional) One or more variables whose values will replace their corresponding
///             placeholders in the template
///
/// # Template Format
///
/// When using variable substitution, the template file should contain placeholders in the
/// format `$variablename`. For example:
///
/// ```text
/// Hello, $name! You are $age years old.
/// ```
///
/// # Returns
///
/// `std::io::Result<String>`
///
/// # Examples
///
/// ```rust
/// // Simple file reading
/// let result = template!("templates/welcome.txt");
/// match result {
///     Ok(content) => println!("{}", content),
///     Err(e) => eprintln!("Failed to read template: {}", e),
/// }
///
/// // With variable substitution
/// let name = "Bob";
/// let role = "admin";
/// let message = template!("templates/user_welcome.txt", name, role).unwrap();
/// // If template contains: "Welcome $name! Your role is $role"
/// // Result will be: "Welcome Bob! Your role is admin"
/// ```
#[macro_export]
macro_rules! template {
    ($path:expr) => {{
        let path: &str = $path;
        std::fs::read_to_string(path)
    }};

    ($path:expr, $($value:ident),+) => {{
        let mut template: String = std::fs::read_to_string($path)?;

        fn replace_in_place(original: &mut String, from: &str, to: &str) {
            *original = original.replace(from, to);
        }

        $(
            replace_in_place(&mut template, &format!("${}", stringify!($value)) ,$value);
        )+

        Ok(template)
    }};
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
