use std::path;

use crate::response::Response;

pub struct Route {
    path: String,
    get: Option<fn() -> Response>,
    post: Option<fn() -> Response>,
}

impl Route {
    pub fn new(path: String) -> Self {
        Self {
            path,
            get: None,
            post: None,
        }
    }

    pub fn get_fn(&mut self, method: &str) -> Option<fn() -> Response> {
        match method {
            "GET" => self.get,
            "POST" => self.post,
            _ => None,
        }
    }

    pub fn set_fn(&mut self, method: &str, f: fn() -> Response) {
        match method {
            "GET" => self.get = Some(f),
            "POST" => self.post = Some(f),
            _ => panic!("Invalid method"),
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

    pub fn add(&mut self, path: &str, method: &str, f: fn() -> Response) {
        let route = self.get_route(path);
        if let Some(r) = route {
            r.set_fn(method, f);
            return;
        }

        let mut new_route = Route::new(path.to_string());
        new_route.set_fn(method, f);
        self.routes.push(new_route);
    }
}
