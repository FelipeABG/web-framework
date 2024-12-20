#![allow(dead_code)]
mod connection;
mod routing;
pub mod server;

#[cfg(test)]
mod tests {

    use super::*;
    use connection::request::Request;
    use connection::response::Response;
    use server::Server;

    #[test]
    fn main_test() {
        let mut server = Server::build("127.0.0.1:8080").unwrap();

        server.add_route("/", index);

        server.run();
    }

    fn index(_request: Request) -> Response {
        Response::html("templates/index.html")
    }
}
