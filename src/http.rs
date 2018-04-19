extern crate rouille;
extern crate serde;
extern crate uuid;

use self::uuid::Uuid;
use self::rouille::Request;
use self::rouille::Response;
use chain::Block;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfo {
    node_id: String,
    current_block_height: u32
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BlocksResponse {
    blocks: Vec<Block>,
    block_height: usize,
}

pub struct Server {
    pub node_id: String,
    pub rusty_chain: Vec<Block>,
}

impl Server {
    pub fn new() -> Server {
        let node_id = Uuid::new_v4().to_string();
        let rusty_chain: Vec<Block> = vec![Block::genesis()];
        Server { node_id, rusty_chain }
    }
}

pub fn route(server: &Server, request: &Request) -> Response {
    match request.url().as_str() {
        "/" => node_info(server),
        "/blocks" => blocks(server),
        _ => Response::text("not found").with_status_code(404)
    }
}

fn node_info(server: &Server) -> Response {
    Response::json(&NodeInfo { node_id: server.node_id.clone(), current_block_height: 0 })
}

fn blocks(server: &Server) -> Response {
    Response::json(&BlocksResponse { blocks: server.rusty_chain.clone(), block_height: server.rusty_chain.len() })
}
