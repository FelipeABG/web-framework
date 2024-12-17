use std::{
    io::{self, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

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
                let data = Self::data_to_string(&tcps);
                let method = Self::get_method(&data);
                let end_point = Self::get_endpoint(&data);
            }
        }
    }

    fn data_to_string(data: &TcpStream) -> String {
        BufReader::new(data)
            .lines()
            .map(|line| line.unwrap())
            .collect()
    }

    fn get_method(data: &str) -> String {
        data.split(" ").next().unwrap().to_string()
    }

    fn get_endpoint(data: &str) -> String {
        data.split(" ").skip(1).next().unwrap().to_string()
    }
}
