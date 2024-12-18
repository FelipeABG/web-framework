pub struct Response {
    content: String,
}

impl Response {
    pub fn as_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }

    pub fn error() -> Self {
        Self {
        content: "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 21\r\n\r\nInternal Server Error".to_string()
        }
    }

    pub fn text(content: &str) -> Self {
        let status_line = "HTTP/1.1 200 OK".to_string();
        let length = content.len();
        Self {
            content: format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"),
        }
    }
}
