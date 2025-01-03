use crate::connection::request::Request;

pub struct Route {
    path: String,
    action: fn(Request) -> String,
}

impl Route {
    pub fn new(path: String, action: fn(Request) -> String) -> Self {
        Self { path, action }
    }

    pub fn get_fn(&mut self) -> fn(Request) -> String {
        return self.action;
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

    pub fn add(&mut self, path: &str, f: fn(Request) -> String) {
        let route = self.get_route(path);
        if let Some(_) = route {
            return;
        }

        let new_route = Route::new(path.to_string(), f);
        self.routes.push(new_route);
    }
}
