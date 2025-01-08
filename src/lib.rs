//! A lightweight HTTP server implementation providing routing, session management,
//! and static file serving capabilities.
//!
//! ## Features
//!
//! - Custom route handling
//! - Static file serving
//! - Session management
//! - TCP connection handling
//! - Custom response methods
//!
//! ## Example
//!
//! ```rust
//! use rwf::Server;
//!
//! fn main() -> std::io::Result<()> {
//!     let mut server = Server::build("127.0.0.1:8080")?;
//!     
//!     // Add a custom route
//!     server.route("/", |_req, _session| {
//!         "Hello, World!".to_string()
//!     });
//!     
//!     // Serve static files from 'css' directory
//!     server.static_dir("css/");
//!     
//!     // Start the server
//!     server.run();
//!     Ok(())
//! }
//! ```

pub mod connection;
mod macros;
mod routing;

use crate::{
    connection::{
        request::Request,
        session::{Session, Sessions},
        RequestHandler,
    },
    routing::Routes,
};
use std::{
    cell::RefCell,
    env::current_dir,
    fs::{read_dir, read_to_string},
    io,
    net::TcpListener,
    path::PathBuf,
    rc::Rc,
};

/// A TCP-based HTTP server with routing and session management capabilities.
pub struct Server {
    listener: TcpListener,
    routes: Rc<RefCell<Routes>>,
    sessions: Rc<RefCell<Sessions>>,
}

impl Server {
    /// Creates a new server instance bound to the specified address.
    ///
    /// # Arguments
    ///
    /// * `addr` - A string slice containing the address in format "host:port"
    ///
    /// # Returns
    ///
    /// * `Result<Server, io::Error>` - A new server instance or an IO error
    ///
    /// # Example
    ///
    /// ```rust
    /// let server = Server::build("127.0.0.1:8080")?;
    /// ```
    pub fn build(addr: &str) -> Result<Self, io::Error> {
        TcpListener::bind(addr).map(|listener| {
            println!("Server built on port: {addr}");
            Self {
                listener,
                routes: Rc::new(RefCell::new(Routes::new())),
                sessions: Rc::new(RefCell::new(Sessions::new())),
            }
        })
    }

    /// Starts the server and begins listening for incoming connections.
    ///
    /// This method runs indefinitely, processing incoming connections and
    /// routing requests to appropriate handlers.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut server = Server::build("127.0.0.1:8080")?;
    /// server.run();
    /// ```
    pub fn run(&mut self) {
        println!("Listening to connections.");
        for conn in self.listener.incoming() {
            if let Ok(request) = conn {
                let mut handler =
                    RequestHandler::new(Rc::clone(&self.routes), Rc::clone(&self.sessions));
                handler.resolve(request)
            }
        }
    }

    /// Registers a new route handler for the specified path.
    /// If the route exists, it does nothing.
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path to match
    /// * `f` - Handler function taking a Request and Session, returning a String response
    ///
    /// # Example
    ///
    /// ```rust
    /// server.route("/hello", |req, session| {
    ///     "Hello, World!".to_string()
    /// });
    /// ```
    pub fn route(&mut self, path: &str, f: fn(Request, &mut Session) -> String) {
        let mut routes = RefCell::borrow_mut(&mut self.routes);
        routes.add(path, f);
    }

    /// Configures static file serving from the specified directory.
    ///
    /// Creates routes for all files in the directory, making them accessible
    /// via HTTP requests.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the directory containing static files
    /// **OBS**: The path must be from the root level of your project.
    ///
    /// # Example
    ///
    /// ```rust
    /// server.static_dir("/static");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the directory cannot be read or if file operations fail.
    pub fn static_dir(&mut self, path: &str) {
        let dir = read_dir(path).unwrap();
        for entry in dir.map(|result| result.unwrap()) {
            let path = format!(
                "/{}/{}",
                path.split("/").last().unwrap(),
                entry.file_name().to_str().unwrap()
            );

            self.route(&path, static_fn);
        }
    }
}

// Handles serving of static files.
///
/// # Arguments
///
/// * `r` - The incoming request
/// * `_` - Unused session parameter
///
/// # Returns
///
/// * `String` - The contents of the requested file
///
/// # Panics
///
/// Panics if the file cannot be found or read.
fn static_fn(r: Request, _: &mut Session) -> String {
    let fname = r.resource.split("/").last().unwrap();
    let fpath = find_file(fname, current_dir().unwrap()).unwrap();
    read_to_string(fpath).unwrap()
}

/// Recursively searches for a file in the directory structure.
///
/// # Arguments
///
/// * `file` - Name of the file to find
/// * `dir_path` - Starting directory for the search
///
/// # Returns
///
/// * `Option<String>` - The path to the found file, if it exists
///
/// # Panics
///
/// Panics if directory operations fail.
fn find_file(file: &str, dir_path: PathBuf) -> Option<String> {
    let dir = read_dir(dir_path).unwrap();
    for entry in dir.map(|rst| rst.unwrap()) {
        let name = entry.file_name();
        let path = entry.path();

        if name.to_str() == Some(file) {
            return Some(path.to_str().unwrap().to_string());
        }

        if path.is_dir() {
            if let Some(found) = find_file(file, path) {
                return Some(found);
            }
        }
    }

    None
}
