extern crate rouille;
extern crate rustychain;
use std::sync::Mutex;

fn main() {
    let server_mutex = Mutex::new(rustychain::http::Server::new());
    rouille::start_server("localhost:8000", move |request| {
        let mut server = server_mutex.lock().unwrap();
        rustychain::http::route(&mut server, request)
    });
}
