use super::request::Request;

pub struct Response {
    content: String,
}
impl Response {
    fn generate_content(length: usize, content: &str) -> String {
        format!("HTTP/1.1 200 OK\r\nContent-Lenght: {length}\r\n\r\n{content}")
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }

    pub fn error() -> Response {
        Response {
            content: "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 21\r\n\r\nResource not found"
                .to_string(),
        }
    }

    pub fn plain_text(content: &str) -> Response {
        let length = content.len();
        println!("Returned plain text: '{content}'");
        Response {
            content: Self::generate_content(length, content),
        }
    }

    pub fn html(path: &str) -> Response {
        match std::fs::read_to_string(path) {
            Ok(file_str) => {
                let length = file_str.len();
                println!("Returned html from '{path}'.");
                Response {
                    content: Self::generate_content(length, file_str.as_str()),
                }
            }
            Err(e) => panic!("{e}"),
        }
    }

    pub fn redirect(arg: Request, f: fn(Request) -> Response) -> Response {
        f(arg)
    }
}
