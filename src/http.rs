extern crate rouille;
extern crate serde;
extern crate uuid;

use self::uuid::Uuid;
use self::rouille::Request;
use self::rouille::Response;
use chain::Block;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfo<'a> {
    node_id: &'a String,
    current_block_height: usize,
}

impl<'a> NodeInfo <'a> {
    pub fn new(server: &'a Server) -> NodeInfo<'a> {
        NodeInfo {
            node_id: &server.node_id,
            current_block_height: server.rusty_chain.len(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BlocksResponse<'a> {
    blocks: &'a Vec<Block>,
    block_height: usize,
}

impl<'a, 'b: 'a> BlocksResponse<'a> {
    pub fn new(server: &'b Server) -> BlocksResponse<'a> {
        BlocksResponse {
            blocks: &server.rusty_chain,
            block_height: server.rusty_chain.len(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MineResponse {
    message: String,
    block: Block,
}

impl MineResponse {
    pub fn new(server: &mut Server) -> MineResponse {
        let (message, block) = server.add_block();

        MineResponse {
            block: block,
            message: message,
        }
    }
}

pub struct Server {
    pub node_id: String,
    pub rusty_chain: Vec<Block>,
}

impl Server {
    pub fn new() -> Server {
        let node_id = Uuid::new_v4().to_string();
        let rusty_chain: Vec<Block> = vec![Block::genesis()];
        Server {
            node_id,
            rusty_chain,
        }
    }

    pub fn add_block(&mut self) -> (String, Block) {
        let mut block;
        let message;

        {
            let previous_block = self.rusty_chain.last().unwrap();
            block = Block::new(vec![], previous_block);
            let (nanos, hash_rate) = block.mine();
            message = format!(
                "Mined a new block in {}ns. Hashing power: {} hashes/s.",
                nanos, hash_rate
            );
        }

        self.rusty_chain.push(block.clone());
        (message, block)
    }
}

pub fn route(server: &mut Server, request: &Request) -> Response {
    match request.url().as_str() {
        "/" => node_info(server),
        "/blocks" => blocks(server),
        "/mine" => mine(server),
        _ => Response::text("not found").with_status_code(404),
    }
}

fn node_info(server: &Server) -> Response {
    Response::json(&NodeInfo::new(server))
}

fn blocks(server: &Server) -> Response {
    Response::json(&BlocksResponse::new(server))
}

fn mine(server: &mut Server) -> Response {
    Response::json(&MineResponse::new(server))
}
