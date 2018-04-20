extern crate rouille;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use self::uuid::Uuid;
use self::rouille::Request;
use self::rouille::Response;
use chain::{Block, Chain, Transaction};
use nodes::*;
use intermediate_transaction::IntermediateTransaction;
use std::io::Read;
use serde_json::Value;
use std::sync::Mutex;

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
    blocks: &'a Chain,
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
    pub fn new(server_mutex: &Mutex<Server>) -> MineResponse {
        let (message, block) = Server::add_block(server_mutex);

        MineResponse {
            block: block,
            message: message,
        }
    }
}

pub struct Server {
    pub node_id: String,
    pub rusty_chain: Chain,
    pub transaction_buffer: Vec<IntermediateTransaction>,
    is_mining: bool,
    pub neighbours: Vec<Node>
}

impl Server {
    pub fn new() -> Server {
        Server {
            node_id: Uuid::new_v4().to_string(),
            rusty_chain: vec![Block::genesis()],
            transaction_buffer: Vec::new(),
            is_mining: false,
            neighbours: Vec::new(),
        }
    }

    pub fn transactions(&self) -> Box<Iterator<Item=Transaction>> {
        let chain = self.rusty_chain.clone();
        Box::new(chain.into_iter().flat_map(|block| block.transactions.clone()))
    }

    pub fn add_block(server_mutex: &Mutex<Server>) -> (String, Block) {
        let mut block;
        let message;
        let transactions: Vec<Transaction>;

        {
            let previous_block = {
                let mut server = server_mutex.lock().unwrap();
                server.is_mining = true;
                transactions = server
                    .transaction_buffer
                    .iter()
                    .take(5)
                    .map(|it| Transaction {
                        id: it.id.clone(),
                        timestamp: it.timestamp,
                        payload: it.payload.clone(),
                    })
                    .collect();
                server.rusty_chain.last().unwrap().clone()
            };
            block = Block::new(transactions, &previous_block);
            let (nanos, hash_rate) = block.mine();
            message = format!(
                "Mined a new block in {}ns. Hashing power: {} hashes/s.",
                nanos, hash_rate
            );
        }

        let mut server = server_mutex.lock().unwrap();
        server.rusty_chain.push(block.clone());
        server.transaction_buffer.retain(|it| block
            .transactions
            .iter()
            .any(|t| t.id != it.id));
        server.is_mining = false;
        (message, block)
    }
}

pub fn route(server: &Mutex<Server>, request: &Request) -> Response {
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

fn node_info(server_mutex: &Mutex<Server>) -> Response {
    let server = server_mutex.lock().unwrap();
    Response::json(&NodeInfo::new(&server))
}

fn blocks(server_mutex: &Mutex<Server>) -> Response {
    let server = server_mutex.lock().unwrap();
    Response::json(&BlocksResponse::new(&server))
}

fn mine(server_mutex: &Mutex<Server>) -> Response {
    let is_mining;
    {
        let server = server_mutex.lock().unwrap();
        is_mining = server.is_mining;
    }
    if is_mining {
        Response::text("Already mining! Come back later").with_status_code(412)
    }
    else {
        Response::json(&MineResponse::new(server_mutex))
    }
}

fn create_transaction(server_mutex: &Mutex<Server>, request: &Request) -> Response {
    let mut server = server_mutex.lock().unwrap();
    let mut data = request.data().unwrap();
    let mut content = String::new();
    data.read_to_string(&mut content).unwrap();
    let payload: Value = serde_json::from_str(&content).unwrap();
    let payload_str = payload.get("payload").unwrap().as_str().unwrap();

    let new_transaction = IntermediateTransaction::new(payload_str);
    server.transaction_buffer.push(new_transaction.clone());

    Response::json(&new_transaction)
}

fn transactions(server_mutex: &Mutex<Server>) -> Response {
    let server = server_mutex.lock().unwrap();
    Response::json(&server.transaction_buffer)
}

fn register_node(server_mutex: &Mutex<Server>, request: &Request) -> Response {
    let mut server = server_mutex.lock().unwrap();
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
