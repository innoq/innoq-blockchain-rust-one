extern crate serde_json;
extern crate crypto_hash;

use serde_json::Value;
use crypto_hash::{Algorithm, hex_digest};

#[derive(Serialize, Deserialize)]
struct Transaction {
    id:String,
    timestamp:u32,
    payload:String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Block {
    index:u32,
    timestamp:u32,
    proof:u32,
    transactions:Vec<Transaction>,
    previous_block_hash:String,
}

fn serialize_genesis_block() -> String {

    let transaction = Transaction{
        id: String::from("b3c973e2-db05-4eb5-9668-3e81c7389a6d"),
        payload: String::from("I am Heribert Innoq"),
        timestamp: 0,
    };

    let block = Block {
        index: 1,
        timestamp: 0,
        proof: 1917336,
        transactions: vec!(transaction),
        previous_block_hash: String::from("0"),
    };

    serde_json::to_string(&block).unwrap()
}

fn hash(json: String) -> String {
    hex_digest(Algorithm::SHA256, json.as_bytes())
}

#[test]
fn test_serialize_genesis_block() {

    assert_eq!("{\"index\":1,\"timestamp\":0,\"proof\":1917336,\"transactions\":[{\"id\":\"b3c973e2-db05-4eb5-9668-3e81c7389a6d\",\"timestamp\":0,\"payload\":\"I am Heribert Innoq\"}],\"previousBlockHash\":\"0\"}", serialize_genesis_block());
}

#[test]
fn test_hash_for_genesis_block() {
    assert_eq!("000000b642b67d8bea7cffed1ec990719a3f7837de5ef0f8ede36537e91cdc0e", hash(serialize_genesis_block()))
}
