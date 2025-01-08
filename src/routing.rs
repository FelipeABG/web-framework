//! # Routing Module
//!
//! Provides routing functionality for the HTTP server, managing URL paths and their
//! associated handler functions.
//!
//! This module implements a simple routing system that maps URL paths to handler
//! functions. It prevents duplicate routes and provides easy access to route handlers.
//!
//! ## Example
//!
//! ```rust
//! use routing::Routes;
//!
//! let mut routes = Routes::new();
//!
//! // Add a route
//! routes.add("/hello", |req, session| {
//!     "Hello, World!".to_string()
//! });
//!
//! // Get a route
//! if let Some(route) = routes.get_route("/hello") {
//!     let handler = route.get_fn();
//!     // Use the handler...
//! }
//! ```

use crate::connection::{request::Request, session::Session};

/// Represents a single route in the routing system.
///
/// A route pairs a URL path with a handler function that processes requests
/// to that path and generates responses.
#[derive(Debug)]
pub struct Route {
    /// The URL path this route responds to
    path: String,
    /// The handler function for this route
    handler: fn(Request, &mut Session) -> String,
}

impl Route {
    /// Creates a new route with the specified path and handler function.
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path this route will match
    /// * `action` - The handler function that will process requests to this path
    ///
    /// # Returns
    ///
    /// A new `Route` instance
    ///
    /// # Example
    ///
    /// ```rust
    /// let route = Route::new(
    ///     "/hello".to_string(),
    ///     |req, session| "Hello, World!".to_string()
    /// );
    /// ```
    pub fn new(path: String, handler: fn(Request, &mut Session) -> String) -> Self {
        Self { path, handler }
    }

    /// Returns the handler function for this route.
    ///
    /// # Returns
    ///
    /// The function that handles requests for this route
    ///
    /// # Example
    ///
    /// ```rust
    /// if let Some(route) = routes.get_route("/hello") {
    ///     let handler = route.get_fn();
    ///     let response = handler(request, session);
    /// }
    /// ```
    pub fn get_fn(&mut self) -> fn(Request, &mut Session) -> String {
        self.handler
    }
}

/// Collection and manager of all routes in the system.
///
/// `Routes` maintains a vector of `Route` instances and provides methods
/// for adding new routes and finding existing ones.
#[derive(Debug)]
pub struct Routes {
    /// Vector storing all registered routes
    routes: Vec<Route>,
}

impl Routes {
    /// Creates a new, empty routing table.
    ///
    /// # Returns
    ///
    /// A new `Routes` instance with no registered routes
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Finds a route matching the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path to look up
    ///
    /// # Returns
    ///
    /// * `Some(&mut Route)` if a matching route is found
    /// * `None` if no matching route exists
    ///
    /// # Example
    ///
    /// ```rust
    /// if let Some(route) = routes.get_route("/hello") {
    ///     // Use the route...
    /// } else {
    ///     // Handle 404...
    /// }
    /// ```
    pub fn get_route(&mut self, path: &str) -> Option<&mut Route> {
        for route in self.routes.iter_mut() {
            if route.path == path {
                return Some(route);
            }
        }
        None
    }

    /// Adds a new route to the routing table.
    ///
    /// If a route with the same path already exists, the function returns
    /// without making any changes.
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path for the new route
    /// * `f` - The handler function for the route
    ///
    /// # Example
    ///
    /// ```rust
    /// routes.add("/hello", |req, session| {
    ///     "Hello, World!".to_string()
    /// });
    /// ```
    pub fn add(&mut self, path: &str, f: fn(Request, &mut Session) -> String) {
        let route = self.get_route(path);
        if let Some(_) = route {
            return;
        }
        let new_route = Route::new(path.to_string(), f);
        self.routes.push(new_route);
    }
}
