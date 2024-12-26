use super::request::Request;

pub fn format_content(length: usize, content: &str) -> Vec<u8> {
    format!("HTTP/1.1 200 OK\r\nContent-Lenght: {length}\r\n\r\n{content}").into_bytes()
}

pub fn error() -> String {
    "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 21\r\n\r\nResource not found".into()
}

pub fn html(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(file_str) => file_str,
        Err(e) => panic!("{e}"),
    }
}

pub fn redirect(arg: Request, f: fn(Request) -> String) -> String {
    f(arg)
}
