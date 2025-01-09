<h1 align="center">Rust web Framework</h1>

<p align="center">A basic web framework written in Rust that provides basic HTTP server functionality with routing and session management.</p>

## Features

- **HTTP Server**: Built on Rust's standard TCP listener functionality
- **Routing System**: Simple path-based routing for handling requests
- **Session Management**: Built-in session handling with cookie support
- **Static File Serving**: Easy serving of static files from directories
- **Request Parsing**: Support for GET and POST methods with body parsing
- **Form Data Processing**: Built-in handling of URL-encoded form data
- **Response Generation**: Flexible response formatting with multiple functions
- **Basic Templating**: Template macro for basic substitution of variables.

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
// Serve files from the 'static' directory
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

### Templating

##### Basic Template Loading

```rust
server.route("/page", |_req, _session| {
    template!("templates/page.html")
        .unwrap_or_else(|_| "Error loading template".to_string())
});
```

##### Variable Substitution

```rust
server.route("/profile", |_req, session| {
    let username = session.get("username").unwrap_or("Guest".to_string());
    let role = session.get("role").unwrap_or("user".to_string());
    template!("templates/profile.html", username, role)
        .unwrap_or_else(|_| "Error loading template".to_string())
});
```

Example template file (profile.html):

```html
<!doctype html>
<html>
  <head>
    <title>Profile Page</title>
  </head>
  <body>
    <h1>Welcome, $username!</h1>
    <p>Your role is: $role</p>
  </body>
</html>
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

### Response

Availible in rwf::response:

- `response::template!` - returns a template with optional variable substitutions
- `response::error404` - return 404 error
- `response::redirect` - redirect a request to another route

## Limitations

- Currently supports only GET and POST methods
- No built-in database integration
- Basic error handling (uses panic in some cases)
- No HTTPS support
- No async/await support

## License

Do not use this! It is poorly organized and designed. It was made for educational purposes only.
But if you want to use it any way, feel free to do so.
