extern crate serde_json;
extern crate crypto_hash;

use crypto_hash::{Algorithm, hex_digest};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    id:String,
    timestamp:u32,
    payload:String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    index:u32,
    timestamp:u32,
    proof:u32,
    transactions:Vec<Transaction>,
    previous_block_hash:String,
}

impl Block {
    pub fn genesis() -> Block {
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

        block
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn hash(&self) -> String {
        hex_digest(Algorithm::SHA256, self.to_json().as_bytes())
    }
}



#[test]
fn test_serialize_genesis_block() {

    let genesis = Block::genesis();

    assert_eq!("{\"index\":1,\"timestamp\":0,\"proof\":1917336,\"transactions\":[{\"id\":\"b3c973e2-db05-4eb5-9668-3e81c7389a6d\",\"timestamp\":0,\"payload\":\"I am Heribert Innoq\"}],\"previousBlockHash\":\"0\"}", genesis.to_json());
}

#[test]
fn test_hash_for_genesis_block() {

    let genesis = Block::genesis();

    assert_eq!("000000b642b67d8bea7cffed1ec990719a3f7837de5ef0f8ede36537e91cdc0e", genesis.hash())
}
