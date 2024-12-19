use crate::response::{self};
use crate::routing::Routes;
use std::{
    cell::RefCell,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    rc::Rc,
};

pub struct RequestHandler {
    routes: Rc<RefCell<Routes>>,
}

impl RequestHandler {
    pub fn routes(routes: Rc<RefCell<Routes>>) -> Self {
        Self { routes }
    }

    pub fn resolve(&mut self, mut request: TcpStream) {
        let data = Self::format_data(&request);
        let method = Self::http_method(&data);
        let path = Self::http_path(&data);

        println!("{method} request on '{path}' received.",);
        let mut routes = RefCell::borrow_mut(&self.routes);
        if let Some(route) = routes.get_route(&path) {
            if let Some(f) = route.get_fn(&method) {
                request.write_all(f().as_bytes()).unwrap();
                return;
            }
        }

        request.write_all(response::error().as_bytes()).unwrap()
    }

    fn format_data(data: &TcpStream) -> String {
        BufReader::new(data)
            .lines()
            .map(|rst| rst.unwrap())
            .take_while(|line| !line.is_empty())
            .collect()
    }

    fn http_method(formatted_data: &str) -> String {
        formatted_data.split(" ").next().unwrap().to_string()
    }

    fn http_path(formatted_data: &str) -> String {
        formatted_data
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .to_string()
    }
}
