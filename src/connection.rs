pub mod method;
pub mod request;
pub mod response;

use crate::routing::Routes;
use request::Request;
use response::Response;
use std::{cell::RefCell, io::Write, net::TcpStream, rc::Rc};

pub struct RequestHandler {
    routes: Rc<RefCell<Routes>>,
}

impl RequestHandler {
    pub fn new(routes: Rc<RefCell<Routes>>) -> Self {
        Self { routes }
    }

    pub fn resolve(&mut self, mut stream: TcpStream) {
        let request = Request::parse(&stream);

        println!("{:?} request on '{}'.", request.method, request.resource);
        let mut routes = RefCell::borrow_mut(&self.routes);
        if let Some(route) = routes.get_route(&request.resource) {
            let f = route.get_fn();
            stream.write_all(f(request).as_bytes()).unwrap();
            return;
        }

        println!("No resource found, returned error");
        stream.write_all(Response::error().as_bytes()).unwrap()
    }
}
