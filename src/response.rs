pub struct Response {
    content: String,
}
impl Response {
    pub fn as_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }
}
const STATUS: &'static str = "HTTP/1.1 200 OK";

pub fn error() -> Response {
    Response {
        content:
            "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 21\r\n\r\nInternal Server Error"
                .to_string(),
    }
}

pub fn plain_text(content: &str) -> Response {
    let length = content.len();
    println!("Returned plain text: '{content}'");
    Response {
        content: format!("{}\r\nContent-Length: {length}\r\n\r\n{content}", STATUS),
    }
}
