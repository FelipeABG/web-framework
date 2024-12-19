#![allow(dead_code)]
mod connection;
pub mod response;
mod routing;
pub mod server;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::response::Response;
    use server::Server;

    #[test]
    fn main_test() {
        let mut server = Server::build("127.0.0.1:8080").unwrap();

        server.add_route("GET", "/", index);
        server.add_route("GET", "/home", home);

        server.run();
    }

    fn index() -> Response {
        Response::html("templates/index.html")
    }

    fn home() -> Response {
        Response::redirect(index)
    }
}
