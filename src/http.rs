extern crate rouille;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use self::uuid::Uuid;
use self::rouille::Request;
use self::rouille::Response;
use chain::{Block, Transaction};
use nodes::*;
use std::io::Read;
use serde_json::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeInfo<'a> {
    node_id: &'a String,
    current_block_height: usize,
    neighbours: Vec<Node>,
}

impl<'a> NodeInfo <'a> {
    pub fn new(server: &'a Server) -> NodeInfo<'a> {
        NodeInfo {
            node_id: &server.node_id,
            current_block_height: server.rusty_chain.len(),
            neighbours: server.neighbours.clone(),
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
    pub transaction_buffer: Vec<Transaction>,
    pub neighbours: Vec<Node>
}

impl Server {
    pub fn new() -> Server {
        Server {
            node_id: Uuid::new_v4().to_string(),
            rusty_chain: vec![Block::genesis()],
            transaction_buffer: Vec::new(),
            neighbours: Vec::new(),
        }
    }

    pub fn transactions(&self) -> Box<Iterator<Item=Transaction>> {
        let chain = self.rusty_chain.clone();
        Box::new(chain.into_iter().flat_map(|block| block.transactions.clone()))
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
        "/transactions"  if (request.method() == "POST")  => create_transaction(server, request),
        "/transactions"  if (request.method() == "GET")  => transactions(server),
        "/transactions"   => Response::text("invalid method").with_status_code(405),
        "/nodes/register" if (request.method() == "POST") => register_node(server, request),
        "/nodes/register" => Response::text("invalid method").with_status_code(405),
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

fn create_transaction(_server: &mut Server, request: &Request) -> Response {
    let mut data = request.data().unwrap();
    let mut content = String::new();
    data.read_to_string(&mut content).unwrap();
    let payload: Value = serde_json::from_str(&content).unwrap();

    println!("{:?}",payload);

    Response::text("Create transactions")
}

fn transactions(server: &Server) -> Response {
    Response::json(&server.transaction_buffer)
}

fn register_node(server: &mut Server, request: &Request) -> Response {
    let mut data = request.data().unwrap();
    let mut content = String::new();
    data.read_to_string(&mut content).unwrap();
    let payload: NodeRegistration = serde_json::from_str(&content).unwrap();
    let node_id = Uuid::new_v4().to_string();
    let node = Node { node_id, host: payload.host };
    server.neighbours.push(node.clone());
    Response::json(&NodeRegistered {
        message: String::from("New node added"),
        node,
    })
}
