extern crate rouille;
extern crate rustychain;

fn main() {
    let server = rustychain::http::Server::new();
    rouille::start_server("localhost:8000", move |request| rustychain::http::route(&server, request));
}
