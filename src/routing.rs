struct Route {
    path: String,
    get: fn() -> String,
    post: fn() -> String,
}

pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
}
