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
