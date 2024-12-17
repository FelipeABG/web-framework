#![allow(dead_code)]

mod server;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn main_test() {
        let server = server::Server::create("127.0.0.1:8080");

        match server {
            Ok(server) => server.run(),
            Err(e) => eprintln!("{e}"),
        }
    }
}
