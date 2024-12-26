pub mod connection;
mod routing;

use crate::{
    connection::{request::Request, RequestHandler},
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
}

impl Server {
    pub fn build(addr: &str) -> Result<Self, io::Error> {
        TcpListener::bind(addr).map(|listener| {
            println!("Server built on port: {addr}");
            Self {
                listener,
                routes: Rc::new(RefCell::new(Routes::new())),
            }
        })
    }

    pub fn run(&mut self) {
        println!("Listening to connections.");
        for conn in self.listener.incoming() {
            if let Ok(request) = conn {
                let mut handler = RequestHandler::new(Rc::clone(&self.routes));
                handler.resolve(request)
            }
        }
    }

    pub fn route(&mut self, path: &str, f: fn(Request) -> String) {
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

fn static_fn(r: Request) -> String {
    let fname = r.resource.split("/").last().unwrap();
    let fpath = find_file(fname, current_dir().unwrap()).unwrap();
    read_to_string(fpath).unwrap()
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
