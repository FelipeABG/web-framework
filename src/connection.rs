//! # Connection Module
//!
//! Handles HTTP request processing, session management, and response generation.
//! This module is responsible for managing TCP connections and routing requests
//! to appropriate handlers.
//!
//! ## Submodules
//!
//! * `method` - HTTP method definitions
//! * `request` - Request parsing and handling
//! * `response` - Response formatting and generation
//! * `session` - Session management

pub mod method;
pub mod request;
pub mod response;
pub mod session;

use crate::routing::Routes;
use request::Request;
use session::Sessions;
use std::{cell::RefCell, io::Write, net::TcpStream, rc::Rc};

/// Handles incoming HTTP requests, manages sessions, and generates responses.
///
/// The `RequestHandler` is responsible for:
/// - Processing incoming TCP streams
/// - Parsing HTTP requests
/// - Managing sessions
/// - Routing requests to appropriate handlers
/// - Generating and sending responses
pub struct RequestHandler {
    routes: Rc<RefCell<Routes>>,
    sessions: Rc<RefCell<Sessions>>,
}

impl RequestHandler {
    pub fn new(routes: Rc<RefCell<Routes>>, sessions: Rc<RefCell<Sessions>>) -> Self {
        Self { routes, sessions }
    }

    /// Processes an incoming TCP stream and generates an appropriate response.
    ///
    /// This method:
    /// 1. Parses the incoming HTTP request
    /// 2. Logs the request details
    /// 3. Attempts to find a matching route
    /// 4. Manages session state
    /// 5. Executes the route handler if found
    /// 6. Generates and sends the response
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream containing the incoming request
    ///
    /// # Side Effects
    ///
    /// * Writes response data to the TCP stream
    /// * Logs request information to stdout
    ///
    /// # Panics
    ///
    /// Panics if writing to the TCP stream fails
    pub fn resolve(&mut self, mut stream: TcpStream) {
        let request = Request::parse(&stream);

        println!("{:?} request on '{}'.", request.method, request.resource);

        let mut routes = RefCell::borrow_mut(&self.routes);
        let mut sessions = RefCell::borrow_mut(&self.sessions);

        if let Some(route) = routes.get_route(&request.resource) {
            let f = route.get_fn();

            let session_id = request
                .session
                .filter(|id| sessions.contains(id))
                .unwrap_or_else(|| sessions.add());

            let session = sessions.get(session_id);

            let response = f(request, session);
            let fmt_response = response::format_content(response.len(), &response, session_id);
            stream.write_all(&fmt_response).unwrap();
            return;
        }

        println!("No resource found, returned error");
        stream.write_all(response::error().as_bytes()).unwrap()
    }
}
