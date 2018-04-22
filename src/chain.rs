extern crate crypto_hash;
extern crate serde_json;

use crypto_hash::{hex_digest, Algorithm};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

const HASH_PREFIX: &str = "000000";

pub type Chain = Vec<Block>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub timestamp: u64,
    pub payload: String,
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

pub fn choose(this: Chain, that: Chain) -> (Chain, Vec<Transaction>) {
    if !validate(&that) || that.len() < this.len() {
        return (this, vec![]);
    }
    let ids: HashSet<String> = transactions(&that).into_iter().map(|t| t.id).collect();
    let mut txs = transactions(&this);
    txs.retain(|tx| ids.contains(&tx.id));
    (that, txs)
}

pub fn transactions(chain: &Chain) -> Vec<Transaction> {
    chain.iter().flat_map(|block| block.transactions.clone()).collect()
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
        Block::genesis_proof(1917336)
    }

    pub fn genesis_proof(proof: u64) -> Block {
        let transaction = Transaction {
            id: String::from("b3c973e2-db05-4eb5-9668-3e81c7389a6d"),
            payload: String::from("I am Heribert Innoq"),
            timestamp: 0,
        };

        Block {
            index: 1,
            timestamp: 0,
            proof,
            transactions: vec![transaction],
            previous_block_hash: String::from("0"),
        }
    }

    pub fn with_proof(&self, proof: u64) -> Block {
        let mut block = self.clone();
        block.proof = proof;
        block
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
#[ignore]
fn test_mine() {
    let previous_block = Block::genesis();
    let mut block = Block::new(Vec::new(), &previous_block);

    let (micros, hash_rate) = block.mine();
    assert_eq!(block.valid(), true);
    println!("{:?}", block);
    println!("{} {}", micros, hash_rate);
}

#[test]
#[ignore]
fn test_genesis_mine() {
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

#[cfg(test)]
mod benchmarks {
    use super::Block;

    use crypto_hash::{digest, hex_digest, Algorithm, Hasher};
    use serde_json;
    use std::io::Write;
    use test::{black_box, Bencher};

    const PREFIX_STRING: &str = "0000";
    const PREFIX_BYTES: &[u8] = &[0, 0];
    const DEFAULT_PROOF: u64 = 0x123456789ABCDEF0;

    #[bench]
    fn bench_mine_loop_string(b: &mut Bencher) {
        let mut genesis = Block::genesis();
        genesis.proof = 0;

        fn validate(block: &Block) -> bool {
            hex_digest(
                Algorithm::SHA256,
                serde_json::to_string(block).unwrap().as_bytes(),
            ).starts_with(PREFIX_STRING)
        }

        b.iter(|| {
            while !validate(&genesis) {
                genesis.proof += 1;
            }
            black_box(genesis.proof);
        });
    }

    #[bench]
    fn bench_mine_loop_bytes_hex(b: &mut Bencher) {
        let mut genesis = Block::genesis();
        genesis.proof = 0;

        fn validate(block: &Block) -> bool {
            hex_digest(Algorithm::SHA256, &serde_json::to_vec(block).unwrap())
                .starts_with(PREFIX_STRING)
        }

        b.iter(|| {
            while !validate(&genesis) {
                genesis.proof += 1;
            }
            black_box(genesis.proof);
        });
    }

    #[bench]
    fn bench_mine_loop_bytes(b: &mut Bencher) {
        let mut genesis = Block::genesis();
        genesis.proof = 0;

        fn validate(block: &Block) -> bool {
            digest(Algorithm::SHA256, &serde_json::to_vec(block).unwrap()).starts_with(PREFIX_BYTES)
        }

        b.iter(|| {
            while !validate(&genesis) {
                genesis.proof += 1;
            }
            black_box(genesis.proof);
        });
    }

    #[bench]
    fn bench_mine_loop_writer(b: &mut Bencher) {
        let mut genesis = Block::genesis();
        genesis.proof = 0;

        fn validate(block: &Block) -> bool {
            let mut h = Hasher::new(Algorithm::SHA256);
            serde_json::to_writer(h.by_ref(), block).unwrap();
            h.finish().starts_with(PREFIX_BYTES)
        }

        b.iter(|| {
            while !validate(&genesis) {
                genesis.proof += 1;
            }
            black_box(genesis.proof);
        });
    }

    #[bench]
    fn bench_mine_iter_bytes(b: &mut Bencher) {
        let genesis = Block::genesis_proof(0);

        fn validate(block: &Block) -> bool {
            digest(Algorithm::SHA256, &serde_json::to_vec(block).unwrap()).starts_with(PREFIX_BYTES)
        }

        b.iter(|| {
            let proof = (0..)
                .find(|&proof| {
                    let block = genesis.with_proof(proof);
                    validate(&block)
                })
                .unwrap();
            black_box(proof);
        });
    }

    #[bench]
    fn bench_mine_iter_bytes_splice(b: &mut Bencher) {
        let genesis = Block::genesis_proof(DEFAULT_PROOF);
        let haystack = serde_json::to_vec(&genesis).unwrap();
        let needle = serde_json::to_vec(&DEFAULT_PROOF).unwrap();
        let slice = needle.as_slice();
        let index = haystack
            .windows(needle.len())
            .position(|w| w == slice)
            .unwrap();
        let range = index..index + needle.len();

        let validate = |proof: &u64| {
            let repl = serde_json::to_vec(&proof).unwrap();
            let mut b = haystack.clone();
            b.splice(range.clone(), repl);
            digest(Algorithm::SHA256, &b).starts_with(PREFIX_BYTES)
        };

        b.iter(|| {
            let proof = (0..).find(validate).unwrap();
            black_box(proof);
        });
    }

    #[bench]
    fn bench_mine_iter_bytes_write(b: &mut Bencher) {
        let genesis = Block::genesis_proof(DEFAULT_PROOF);
        let haystack = serde_json::to_vec(&genesis).unwrap();
        let needle = serde_json::to_vec(&DEFAULT_PROOF).unwrap();
        let slice = needle.as_slice();
        let index = haystack
            .windows(needle.len())
            .position(|w| w == slice)
            .unwrap();
        let (prefix, temp) = haystack.split_at(index);
        let (_, suffix) = temp.split_at(needle.len());

        let validate = |proof: &u64| {
            let mut h = Hasher::new(Algorithm::SHA256);
            h.write_all(prefix).unwrap();
            h.write_all(&serde_json::to_vec(&proof).unwrap()).unwrap();
            h.write_all(suffix).unwrap();
            h.finish().starts_with(PREFIX_BYTES)
        };

        b.iter(|| {
            let proof = (0..).find(validate).unwrap();
            black_box(proof);
        });
    }
}
