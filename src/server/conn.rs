use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub struct ConnHandler {
    method: String,
    path: String,
    conn: TcpStream,
}

impl ConnHandler {
    pub fn new(conn: TcpStream) -> Self {
        let str_data = Self::data_to_string(&conn);
        Self {
            conn,
            method: Self::get_method(&str_data),
            path: Self::get_endpoint(&str_data),
        }
    }

    pub fn resolve(&mut self) {
        let template = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
        self.conn.write_all(template).unwrap()
    }

    fn data_to_string(data: &TcpStream) -> String {
        BufReader::new(data)
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect()
    }

    fn get_method(data: &str) -> String {
        data.split(" ").next().unwrap().to_string()
    }

    fn get_endpoint(data: &str) -> String {
        data.split(" ").skip(1).next().unwrap().to_string()
    }
}
