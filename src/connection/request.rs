use std::io::Read;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use super::method::Method;

pub struct Request {
    pub resource: String,
    pub method: Method,
    pub header: String,
    pub body: Option<String>,
}

impl Request {
    pub fn parse(stream: &TcpStream) -> Self {
        let (header, body) = Self::get_data(stream);
        let method = Self::http_method(&header);
        let path = Self::http_path(&header);

        Self {
            resource: path,
            method,
            header,
            body,
        }
    }

    fn get_data(stream: &TcpStream) -> (String, Option<String>) {
        let mut buffer = BufReader::new(stream);
        let mut header = Vec::new();
        let mut content_length = 0;

        loop {
            let mut line = String::new();
            buffer.read_line(&mut line).unwrap();
            let line = line.trim_end();

            if line.is_empty() {
                break;
            }

            if line.starts_with("Content-Length: ") {
                content_length = line.split(": ").nth(1).unwrap_or("0").parse().unwrap_or(0);
            }

            header.push(line.to_string());
        }

        let header_str = header.join("\n");
        if content_length == 0 {
            return (header_str, None);
        }

        let mut body = vec![0u8; content_length];
        buffer.read_exact(&mut body).unwrap();
        let body_str = String::from_utf8_lossy(&body).to_string();
        (header_str, Some(body_str))
    }

    fn http_method(formatted_data: &str) -> Method {
        Method::from_string(formatted_data.split(" ").next().unwrap())
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
