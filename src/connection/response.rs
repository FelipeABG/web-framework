use std::usize;

use super::{request::Request, session::Session};

pub fn format_content(length: usize, content: &str, session_id: usize) -> Vec<u8> {
    format!("HTTP/1.1 200 OK\r\nSet-Cookie: session_id={session_id}; HttpOnly\r\nContent-Lenght: {length}\r\n\r\n{content}").into_bytes()
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

pub fn redirect(
    req: Request,
    sess: &mut Session,
    f: fn(Request, &mut Session) -> String,
) -> String {
    f(req, sess)
}
