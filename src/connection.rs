pub mod method;
pub mod request;
pub mod response;
pub mod session;

use crate::routing::Routes;
use request::Request;
use session::Sessions;
use std::{cell::RefCell, io::Write, net::TcpStream, rc::Rc};

pub struct RequestHandler {
    routes: Rc<RefCell<Routes>>,
    sessions: Rc<RefCell<Sessions>>,
}

impl RequestHandler {
    pub fn new(routes: Rc<RefCell<Routes>>, sessions: Rc<RefCell<Sessions>>) -> Self {
        Self { routes, sessions }
    }

    pub fn resolve(&mut self, mut stream: TcpStream) {
        let request = Request::parse(&stream);

        println!("{:?} request on '{}'.", request.method, request.resource);
        let mut routes = RefCell::borrow_mut(&self.routes);
        let mut sessions = RefCell::borrow_mut(&self.sessions);
        if let Some(route) = routes.get_route(&request.resource) {
            let f = route.get_fn();
            match request.session {
                Some(session_id) => {
                    let session = sessions.get_session(session_id);
                    let response = f(request, session);
                    let formatted_response =
                        response::format_content(response.len(), &response, session_id);
                    stream.write_all(&formatted_response).unwrap();
                }
                None => {
                    let session_id = sessions.add_session();
                    let session = sessions.get_session(session_id);
                    let response = f(request, session);
                    let formatted_response =
                        response::format_content(response.len(), &response, session_id);
                    stream.write_all(&formatted_response).unwrap();
                }
            }
            return;
        }

        println!("No resource found, returned error");
        stream.write_all(response::error().as_bytes()).unwrap()
    }
}
