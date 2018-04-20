extern crate crypto_hash;
extern crate serde_json;

use crypto_hash::{hex_digest, Algorithm};
use std::time::{SystemTime, UNIX_EPOCH};

const HASH_PREFIX: &str = "0000";

pub type Chain = Vec<Block>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    id: String,
    timestamp: u64,
    payload: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    index: u64,
    timestamp: u64,
    proof: u64,
    pub transactions: Vec<Transaction>,
    previous_block_hash: String,
}

pub fn current_timestamp() -> u64 {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

    duration_since_epoch.as_secs()
}

pub fn validate(chain: &Chain) -> bool {
    let mut hash = String::from("0");
    for block in chain.iter() {
        if hash != block.previous_block_hash {
            return false;
        }
        if block.transactions.len() > 5 {
            return false;
        }
        hash = block.hash();
        if !hash.starts_with(HASH_PREFIX) {
            return false;
        }
    }
    true
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        Block {
            index: previous_block.index + 1,
            timestamp: current_timestamp(),
            proof: 0,
            transactions: transactions,
            previous_block_hash: previous_block.hash(),
        }
    }

    pub fn genesis() -> Block {
        let transaction = Transaction {
            id: String::from("b3c973e2-db05-4eb5-9668-3e81c7389a6d"),
            payload: String::from("I am Heribert Innoq"),
            timestamp: 0,
        };

        Block {
            index: 1,
            timestamp: 0,
            proof: 1917336,
            transactions: vec![transaction],
            previous_block_hash: String::from("0"),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// This function calculates the hash of this block.
    ///
    /// The Genesis block should have a known hash:
    /// ```
    /// use rustychain::chain::Block;
    /// let hash = Block::genesis().hash();
    /// assert_eq!(hash, "000000b642b67d8bea7cffed1ec990719a3f7837de5ef0f8ede36537e91cdc0e")
    /// ```
    pub fn hash(&self) -> String {
        hex_digest(Algorithm::SHA256, self.to_json().as_bytes())
    }

    pub fn valid(&self) -> bool {
        self.hash().starts_with(HASH_PREFIX)
    }

    pub fn mine(&mut self) -> (u64, u64) {
        let start = SystemTime::now();

        while !self.valid() {
            self.proof += 1
        }

        let end = SystemTime::now();

        let duration = end.duration_since(start).unwrap();
        let nanos: u64 = duration.as_secs() * 1_000_000_000 + (duration.subsec_nanos() as u64);

        let hash_rate = (self.proof * 1_000_000_000) / nanos;

        (nanos, hash_rate)
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

    assert_eq!(
        "000000b642b67d8bea7cffed1ec990719a3f7837de5ef0f8ede36537e91cdc0e",
        genesis.hash()
    )
}

#[test]
fn test_new_empty_block() {
    let previous_block = Block::genesis();
    let block = Block::new(Vec::new(), &previous_block);

    assert_eq!(block.transactions.len(), 0);
    assert_eq!(block.index, 2);
    assert_eq!(block.previous_block_hash, previous_block.hash());
}

#[test]
fn test_validity() {
    let genesis_block = Block::genesis();

    assert_eq!(genesis_block.valid(), true);
}

#[test]
fn test_mining() {
    let previous_block = Block::genesis();
    let mut block = Block::new(Vec::new(), &previous_block);

    let (micros, hash_rate) = block.mine();
    assert_eq!(block.valid(), true);
    println!("{:?}", block);
    println!("{} {}", micros, hash_rate);
}

#[test]
fn test_genesis_hash() {
    let mut genesis = Block::genesis();
    genesis.proof = 0;

    let (time, rate) = genesis.mine();

    println!("{:?}", genesis);
    println!("time: {}, rate: {}", time, rate);
}

#[test]
fn test_validate_genesis() {
    let genesis = Block::genesis();
    let chain = vec![genesis];
    assert!(validate(&chain));
}

#[test]
fn test_validate_mined() {
    let genesis = Block::genesis();
    let mut block = Block::new(Vec::new(), &genesis);
    block.mine();
    let chain = vec![genesis, block];
    assert!(validate(&chain));
}

#[test]
fn test_validate_unmined() {
    let genesis = Block::genesis();
    let block = Block::new(Vec::new(), &genesis);
    let chain = vec![genesis, block];
    assert!(!validate(&chain));
}

#[test]
fn test_validate_too_many_transactions() {
    fn tx(id: u64) -> Transaction {
        Transaction {
            id: format!("{}", id),
            timestamp: id,
            payload: format!("{}", id),
        }
    }
    let transactions = (1..=6).map(tx).collect();
    let genesis = Block::genesis();
    let mut block = Block::new(transactions, &genesis);
    block.mine();
    let chain = vec![genesis, block];
    assert!(!validate(&chain));
}
