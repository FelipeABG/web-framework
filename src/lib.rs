#![allow(dead_code)]
pub mod connection;
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
        server.route("/", index);
        server.route("/home", home);
        server.static_dir("templates/static");
        server.run();
    }

    fn index(_: Request) -> Response {
        Response::html("templates/index.html")
    }

    fn home(r: Request) -> Response {
        Response::redirect(r, index)
    }
}
