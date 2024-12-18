use crate::{connection::RequestHandler, routing::Routes};
use std::{cell::RefCell, io, net::TcpListener, rc::Rc};

pub struct Server {
    listener: TcpListener,
    routes: Rc<RefCell<Routes>>,
}

impl Server {
    fn build(addr: &str) -> Result<Self, io::Error> {
        TcpListener::bind(addr).map(|listener| Self {
            listener,
            routes: Rc::new(RefCell::new(Routes::new())),
        })
    }

    fn run(&mut self) {
        for conn in self.listener.incoming() {
            if let Ok(request) = conn {
                let mut handler = RequestHandler::routes(Rc::clone(&self.routes));
                handler.resolve(request)
            }
        }
    }
}
