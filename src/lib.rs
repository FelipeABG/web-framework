pub mod connection;
mod routing;
pub mod state;

use state::Context;

use crate::{
    connection::{request::Request, response::Response, RequestHandler},
    routing::Routes,
};
use std::{
    cell::RefCell,
    env::current_dir,
    fs::{read_dir, read_to_string},
    io,
    net::TcpListener,
    path::PathBuf,
    rc::Rc,
};

pub struct Server {
    listener: TcpListener,
    routes: Rc<RefCell<Routes>>,
    context: Rc<RefCell<Context>>,
}

impl Server {
    pub fn build(addr: &str) -> Result<Self, io::Error> {
        TcpListener::bind(addr).map(|listener| Self {
            listener,
            routes: Rc::new(RefCell::new(Routes::new())),
            context: Rc::new(RefCell::new(Context::new())),
        })
    }

    pub fn run(&mut self) {
        for conn in self.listener.incoming() {
            if let Ok(request) = conn {
                let mut handler =
                    RequestHandler::new(Rc::clone(&self.routes), Rc::clone(&self.context));
                handler.resolve(request)
            }
        }
    }

    pub fn route(&mut self, path: &str, f: fn(Request, &mut Context) -> Response) {
        let mut routes = RefCell::borrow_mut(&mut self.routes);
        routes.add(path, f);
    }

    pub fn static_dir(&mut self, path: &str) {
        let dir = read_dir(path).unwrap();
        for entry in dir.map(|result| result.unwrap()) {
            let path = format!(
                "/{}/{}",
                path.split("/").last().unwrap(),
                entry.file_name().to_str().unwrap()
            );

            self.route(&path, static_fn);
        }
    }
}

fn static_fn(r: Request, _: &mut Context) -> Response {
    let fname = r.resource.split("/").last().unwrap();
    let fpath = find_file(fname, current_dir().unwrap()).unwrap();
    let content = read_to_string(fpath).unwrap();
    Response::plain_text(&content)
}

fn find_file(file: &str, dir_path: PathBuf) -> Option<String> {
    let dir = read_dir(dir_path).unwrap();
    for entry in dir.map(|rst| rst.unwrap()) {
        let name = entry.file_name();
        let path = entry.path();

        if name.to_str() == Some(file) {
            return Some(path.to_str().unwrap().to_string());
        }

        if path.is_dir() {
            if let Some(found) = find_file(file, path) {
                return Some(found);
            }
        }
    }

    None
}
