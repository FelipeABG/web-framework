# Rust Web Framework

A basic web framework written in Rust that provides basic HTTP server functionality with routing and session management.

## Features

- **HTTP Server**: Built on Rust's standard TCP listener functionality
- **Routing System**: Simple path-based routing for handling requests
- **Session Management**: Built-in session handling with cookie support
- **Static File Serving**: Easy serving of static files from directories
- **Request Parsing**: Support for GET and POST methods with body parsing
- **Form Data Processing**: Built-in handling of URL-encoded form data
- **Response Generation**: Flexible response formatting with session cookies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rwf = { git = "https://github.com/FelipeABG/web-framework" }
```

## Quick Start

Here's a simple example of creating a web server with routes:

```rust
use rwf::Server;

fn main() -> std::io::Result<()> {
    // Create a new server on localhost:8080
    let mut server = Server::build("127.0.0.1:8080")?;
    
    // Add a simple route
    server.route("/hello", |_req, _session| {
        "Hello, World!".to_string()
    });
    
    // Serve static files from a directory
    server.static_dir("templates/static");
    
    // Start the server
    server.run();
    
    Ok(())
}
```

## Usage Examples

### Basic Routing

```rust
server.route("/", |_req, _session| {
    "Welcome to the homepage!".to_string()
});

server.route("/about", |_req, _session| {
    "About page content".to_string()
});
```

### Handling Forms

```rust
use rwf::connection::request::from_forms;

server.route("/submit", |req, _session| {
    if let Some(body) = req.body {
        let form_data = from_forms(&body);
        format!("Received data: {:?}", form_data)
    } else {
        "No data received".to_string()
    }
});
```

### Serving Static Files

```rust
// Serve files from the 'public' directory
server.static_dir("templates/static");

// Files in template/static/main.css will be available at /static/main.css
```

### Session Management

```rust
server.route("/profile", |req, session| {
    if let Some(user_id) = session.get("user_id") {
        format!("Welcome back, user {}", user_id)
    } else {
        "Please log in".to_string()
    }
});
```

## API Reference

### Server

- `Server::build(addr: &str)` - Create a new server instance
- `server.route(path: &str, handler: fn)` - Register a route
- `server.static_dir(path: &str)` - Serve static files
- `server.run()` - Start the server

### Request

Available in the request handler through the first parameter:
- `request.method` - HTTP method (GET/POST)
- `request.resource` - Requested path
- `request.body` - Optional request body
- `request.header` - Raw request headers
- `request.session` - Session ID if present

### Session

Available in the request handler through the second parameter:
- `session.get(key: &str)` - Get a session value
- `session.set(key: String, value: String)` - Set a session value

## Limitations

- Currently supports only GET and POST methods
- No built-in database integration
- Basic error handling (uses panic in some cases)
- No HTTPS support
- No async/await support

## TODO

1. Add support for additional HTTP methods
2. Implement proper error handling
3. Add HTTPS support
4. Add async/await support
5. Implement middleware system
6. Add database integration
7. Improve session security
8. Add request body size limits
9. Implement proper logging

