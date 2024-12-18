use crate::routing::Routes;
use std::{cell::RefCell, net::TcpStream, rc::Rc};

pub struct RequestHandler {
    routes: Rc<RefCell<Routes>>,
}

impl RequestHandler {
    pub fn routes(routes: Rc<RefCell<Routes>>) -> Self {
        Self { routes }
    }

    pub fn resolve(&mut self, request: TcpStream) {}
}
