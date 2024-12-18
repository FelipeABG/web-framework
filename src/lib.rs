#![allow(dead_code)]
mod connection;
mod response;
mod routing;
mod server;

#[cfg(test)]
mod tests {

    use super::*;
    use response::Response;
    use server::Server;

    #[test]
    fn main_test() {
        let mut server = Server::build("127.0.0.1:8080").unwrap();

        server.add_route("GET", "/", index);

        server.run();
    }

    fn index() -> Response {
        Response::text("HOLY SHIT")
    }
}
