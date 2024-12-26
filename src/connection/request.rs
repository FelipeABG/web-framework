use std::collections::HashMap;
use std::io::Read;
use std::usize;
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
    pub session: Option<usize>,
}

impl Request {
    pub fn parse(stream: &TcpStream) -> Self {
        let (header, body) = Self::get_data(stream);
        let method = Self::http_method(&header);
        let path = Self::http_path(&header);
        let session = Self::get_session(&header);

        Self {
            resource: path,
            method,
            header,
            body,
            session,
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

    fn http_method(header_str: &str) -> Method {
        Method::from_string(header_str.split(" ").next().unwrap())
    }

    fn http_path(header_str: &str) -> String {
        header_str.split(" ").skip(1).next().unwrap().to_string()
    }

    pub fn from_forms(body: &String) -> HashMap<String, String> {
        body.split("&")
            .map(|pair| {
                let parts: Vec<_> = pair.split("=").collect();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect()
    }

    pub fn get_session(header_str: &str) -> Option<usize> {
        for line in header_str.lines() {
            if line.contains("session_id") {
                return line.split("=").nth(1).map(|id| id.parse().unwrap());
            }
        }
        None
    }
}
