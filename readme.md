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

Here's a simple showcase of what the framework can do.

```rust
use rwf::connection::method::Method;
use rwf::connection::request::from_forms;
use rwf::connection::response;
use rwf::template;
use rwf::{connection::response::redirect, Server};

fn main() -> Result<(), std::io::Error> {
    // Initialize server on localhost port 8080
    let mut server = Server::build("127.0.0.1:8080")?;

    // Configure static file serving directory for styles
    server.static_dir("templates/styles");

    // Define route handler for root path "/"
    server.route("/", |request, session| match request.method {
        Method::GET => {
            // Check if user session exists
            if let None = session.get::<(String, String)>("user") {
                // If no session exists, redirect to login page
                return redirect("/login");
            };

            // Retrieve user credentials from session
            let (username, password) = session.get::<(String, String)>("user").unwrap();

            // Render home template with user credentials
            template!("templates/home.html", username, password)
        }
        // Return 404 error for any POST requests to root
        Method::POST => response::error404(),
    });

    server.route("/login", |request, session| match request.method {
        Method::GET => {
            // Render login form template
            template!("templates/login.html")
        }
        // Handle POST requests to login path (form submission)
        Method::POST => {
            // Parse form data from request body
            let forms = from_forms(&request.body.unwrap());

            // Extract username and password from form data
            let username = forms.get("username").unwrap();
            let password = forms.get("password").unwrap();

            // Create new session with user credentials
            session.add("user".to_string(), (username.clone(), password.clone()));

            // Redirect to home page after successful login
            redirect("/")
        }
    });

    // Start the server
    server.run();

    // Return Ok if server starts successfully
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
});
```

##### Variable Substitution

```rust
server.route("/profile", |_req, session| {
    let username = session.get("username").unwrap_or("Guest".to_string());
    let role = session.get("role").unwrap_or("user".to_string());
    template!("templates/profile.html", username, role)
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
- `session.add(key: String, value: String)` - Add a session value

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
