//! # Request Module
//!
//! Handles HTTP request parsing and processing, including header parsing,
//! body extraction, and form data processing.
//!
//! This module provides functionality to:
//! - Parse raw TCP streams into structured HTTP requests
//! - Extract HTTP methods and paths
//! - Process request headers and bodies
//! - Handle session management
//! - Parse form data
//!
//! ## Example
//!
//! ```rust
//! use request::Request;
//!
//! // Assuming we have a TcpStream...
//! let request = Request::parse(&stream);
//!
//! // Access request components
//! println!("Method: {:?}", request.method);
//! println!("Path: {}", request.resource);
//! ```

use super::method::Method;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::usize;

/// Represents an HTTP request with all its components.
///
/// Stores parsed information from an HTTP request including the method,
/// resource path, headers, body, and session information.
#[derive(Debug)]
pub struct Request {
    pub resource: String,
    pub method: Method,
    pub header: String,
    pub body: Option<String>,
    pub session: Option<usize>,
}

impl Request {
    /// Parses a TCP stream into a structured HTTP request.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream containing the raw HTTP request
    ///
    /// # Returns
    ///
    /// A new `Request` instance containing the parsed data
    ///
    /// # Example
    ///
    /// ```rust
    /// let request = Request::parse(&tcp_stream);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - Header parsing fails
    /// - Content-Length parsing fails
    /// - UTF-8 conversion fails
    pub fn parse(stream: &TcpStream) -> Self {
        let (header, body) = Self::get_data(stream);
        let method = Self::http_method(&header);
        let path = Self::http_path(&header);
        let session = Self::get_session(&header);
        Self {
            resource: path,
            method,
            header,
            body,
            session,
        }
    }

    /// Extracts header and body data from a TCP stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream to read from
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - The header as a String
    /// - The body as an Option<String>
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - Reading from the stream fails
    /// - Content-Length parsing fails
    /// - UTF-8 conversion fails
    fn get_data(stream: &TcpStream) -> (String, Option<String>) {
        let mut buffer = BufReader::new(stream);
        let mut header = Vec::new();
        let mut content_length = 0;

        loop {
            let mut line = String::new();
            buffer.read_line(&mut line).unwrap();
            let line = line.trim_end();

            if line.is_empty() {
                break;
            }

            if line.starts_with("Content-Length: ") {
                content_length = line.split(": ").nth(1).unwrap_or("0").parse().unwrap_or(0);
            }

            header.push(line.to_string());
        }

        let header_str = header.join("\n");
        if content_length == 0 {
            return (header_str, None);
        }

        let mut body = vec![0u8; content_length];
        buffer.read_exact(&mut body).unwrap();
        let body_str = String::from_utf8_lossy(&body).to_string();
        (header_str, Some(body_str))
    }

    /// Extracts the HTTP method from the header string.
    ///
    /// # Arguments
    ///
    /// * `header_str` - The raw header string
    ///
    /// # Returns
    ///
    /// The parsed HTTP method
    ///
    /// # Panics
    ///
    /// Panics if the method string is invalid or missing
    fn http_method(header_str: &str) -> Method {
        Method::from_string(header_str.split(" ").next().unwrap())
    }

    /// Extracts the request path from the header string.
    ///
    /// # Arguments
    ///
    /// * `header_str` - The raw header string
    ///
    /// # Returns
    ///
    /// The request path as a String
    ///
    /// # Panics
    ///
    /// Panics if the path is missing from the header
    fn http_path(header_str: &str) -> String {
        header_str.split(" ").skip(1).next().unwrap().to_string()
    }

    /// Extracts the session ID from the header string.
    ///
    /// # Arguments
    ///
    /// * `header_str` - The raw header string
    ///
    /// # Returns
    ///
    /// * `Option<usize>` - The session ID if present
    pub fn get_session(header_str: &str) -> Option<usize> {
        for line in header_str.lines() {
            if line.contains("session_id") {
                return line
                    .split("=")
                    .nth(1)
                    .map(|id| id.parse().unwrap_or(100000000000));
            }
        }
        None
    }
}

/// Parses form data from a request body string.
///
/// Converts URL-encoded form data into a key-value HashMap.
///
/// # Arguments
///
/// * `body` - The form data string (e.g., "name=john&age=30")
///
/// # Returns
///
/// A HashMap containing the parsed form fields and values
///
/// # Example
///
/// ```rust
/// let body = "username=john&password=secret".to_string();
/// let form_data = from_forms(&body);
/// assert_eq!(form_data.get("username"), Some(&"john".to_string()));
/// ```
///
/// # Panics
///
/// Panics if the form data is malformed
pub fn from_forms(body: &String) -> HashMap<String, String> {
    body.split("&")
        .map(|pair| {
            let parts: Vec<_> = pair.split("=").collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect()
}
