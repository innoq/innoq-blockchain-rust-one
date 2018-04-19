extern crate rouille;
extern crate rustychain;

fn main() {
    rouille::start_server("localhost:8000", rustychain::http::route);
}
