use core::panic;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

impl Method {
    pub fn from_string(string: &str) -> Self {
        match string {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => panic!("Failed to parse string to http method"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Method::GET => "GET".to_string(),
            Method::POST => "POST".to_string(),
        }
    }
}
