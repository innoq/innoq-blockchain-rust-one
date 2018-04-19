extern crate rouille;
extern crate rustychain;
extern crate uuid;

use uuid::Uuid;

fn main() {
    let node_id = Uuid::new_v4().to_string();
    let server = rustychain::http::Server { node_id };
    rouille::start_server("localhost:8000", move |request| rustychain::http::route(&server, request));
}
