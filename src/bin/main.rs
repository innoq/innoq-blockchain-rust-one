extern crate rouille;
extern crate rustychain;
use std::sync::Mutex;

fn main() {
    let server_mutex = Mutex::new(rustychain::http::Server::new());
    rouille::start_server("localhost:8000", move |request| {
        rustychain::http::route(&server_mutex, request)
    });
}
