extern crate serde_json;

use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Transaction {
    id:String,
    payload:String,
    timestamp:u32
}

#[derive(Serialize, Deserialize)]
struct Block {
    index:u32,
    timestamp:u32,
    proof:u32,
    transactions:Vec<Transaction>,
    previous_block_hash:String
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

#[test]
fn myfirsttest() {

    assert_eq!("", serialize_genesis_block());
}
