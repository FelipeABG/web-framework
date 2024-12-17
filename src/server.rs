mod conn;

use conn::ConnHandler;
use std::io;
use std::net::TcpListener;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn create(addr: &str) -> Result<Self, io::Error> {
        TcpListener::bind(addr).map(|listener| Self { listener })
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            if let Ok(tcps) = stream {
                let mut handler = ConnHandler::new(tcps);
                handler.resolve();
            }
        }
    }
}
