use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use super::method::Method;

pub struct Request {
    pub resource: String,
    pub method: Method,
    pub body: String,
}

impl Request {
    pub fn parse(stream: &TcpStream) -> Self {
        let data = Self::format_data(stream);
        let method = Self::http_method(&data);
        let path = Self::http_path(&data);

        Self {
            resource: path,
            method,
            body: data,
        }
    }

    fn format_data(data: &TcpStream) -> String {
        BufReader::new(data)
            .lines()
            .map(|rst| rst.unwrap())
            .take_while(|line| !line.is_empty())
            .collect()
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
