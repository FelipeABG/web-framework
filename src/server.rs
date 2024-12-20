use crate::{
    connection::{request::Request, response::Response, RequestHandler},
    routing::Routes,
};
use std::{cell::RefCell, io, net::TcpListener, rc::Rc};

pub struct Server {
    listener: TcpListener,
    routes: Rc<RefCell<Routes>>,
}

impl Server {
    pub fn build(addr: &str) -> Result<Self, io::Error> {
        TcpListener::bind(addr).map(|listener| Self {
            listener,
            routes: Rc::new(RefCell::new(Routes::new())),
        })
    }

    pub fn run(&mut self) {
        for conn in self.listener.incoming() {
            if let Ok(request) = conn {
                let mut handler = RequestHandler::routes(Rc::clone(&self.routes));
                handler.resolve(request)
            }
        }
    }

    pub fn route(&mut self, path: &str, f: fn(Request) -> Response) {
        let mut routes = RefCell::borrow_mut(&mut self.routes);
        routes.add(path, f);
    }

    pub fn source_dir() {
        todo!()
    }
}
