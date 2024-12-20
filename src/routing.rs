use core::panic;

use crate::connection::request::Request;
use crate::connection::response::Response;

pub struct Route {
    path: String,
    action: Option<fn(Request) -> Response>,
}

impl Route {
    pub fn new(path: String) -> Self {
        Self { path, action: None }
    }

    pub fn get_fn(&mut self) -> Option<fn(Request) -> Response> {
        return self.action;
    }

    pub fn set_fn(&mut self, f: fn(Request) -> Response) {
        match self.action {
            Some(_) => panic!("Resource already defined."),
            None => self.action = Some(f),
        }
    }
}

pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn get_route(&mut self, path: &str) -> Option<&mut Route> {
        for route in self.routes.iter_mut() {
            if route.path == path {
                return Some(route);
            }
        }
        None
    }

    pub fn add(&mut self, path: &str, f: fn(Request) -> Response) {
        let route = self.get_route(path);
        if let Some(r) = route {
            r.set_fn(f);
            return;
        }

        let mut new_route = Route::new(path.to_string());
        new_route.set_fn(f);
        self.routes.push(new_route);
    }
}
