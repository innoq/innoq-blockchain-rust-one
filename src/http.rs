extern crate rouille;
extern crate serde;

use self::rouille::Request;
use self::rouille::Response;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfo {
    node_id: String,
    current_block_height: u32
}

pub fn route(request: &Request) -> Response {
    match request.url().as_str() {
        "/" => rouille::Response::json(&NodeInfo { node_id: "fobbar".to_owned(), current_block_height: 0 }),
        "/blocksss" => rouille::Response::text("blocks"),
        _ => rouille::Response::text("not found").with_status_code(404)
    }
}
