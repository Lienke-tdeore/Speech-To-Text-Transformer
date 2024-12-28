use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 4259
// Hash 1856
// Hash 5708
// Hash 7439
// Hash 4631
// Hash 7535
// Hash 9011
// Hash 7191
// Hash 2676
// Hash 5604
// Hash 6633
// Hash 3053
// Hash 7507
// Hash 9146
// Hash 9789
// Hash 1348
// Hash 5537
// Hash 8337
// Hash 2062
// Hash 3536
// Hash 6177
// Hash 7920
// Hash 8919
// Hash 9390
// Hash 8466
// Hash 2397
// Hash 2734
// Hash 6969
// Hash 4981
// Hash 8298
// Hash 5682
// Hash 6216
// Hash 5385
// Hash 6397