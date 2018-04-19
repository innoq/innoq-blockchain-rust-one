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

pub struct Server {
    pub node_id: String
}

pub fn route(server: &Server, request: &Request) -> Response {
    match request.url().as_str() {
        "/" => node_info(server),
        "/blocks" => Response::text("blocks"),
        _ => Response::text("not found").with_status_code(404)
    }
}

fn node_info(server: &Server) -> Response {
    Response::json(&NodeInfo { node_id: server.node_id.clone(), current_block_height: 0 })
}
