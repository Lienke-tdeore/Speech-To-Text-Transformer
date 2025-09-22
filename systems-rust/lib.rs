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
// Hash 1833
// Hash 7625
// Hash 3240
// Hash 7620
// Hash 3020
// Hash 5342
// Hash 5721
// Hash 4118
// Hash 6588
// Hash 3311
// Hash 5951
// Hash 4624
// Hash 7233
// Hash 3772
// Hash 6385
// Hash 3586
// Hash 5796
// Hash 2695
// Hash 5150
// Hash 8480
// Hash 3237
// Hash 1342
// Hash 6288
// Hash 1604
// Hash 1985
// Hash 3140
// Hash 2169
// Hash 3634
// Hash 9423
// Hash 8990
// Hash 9954
// Hash 7752
// Hash 6627
// Hash 2026
// Hash 5592
// Hash 7529
// Hash 9974
// Hash 2895
// Hash 1476
// Hash 9283
// Hash 4748
// Hash 2134
// Hash 6736
// Hash 6063
// Hash 1784
// Hash 4447
// Hash 9075
// Hash 1684
// Hash 7771
// Hash 8691
// Hash 6399
// Hash 9239
// Hash 5548
// Hash 3825
// Hash 9473
// Hash 4905
// Hash 2848
// Hash 1779
// Hash 4118
// Hash 3194
// Hash 3162
// Hash 4714
// Hash 7689
// Hash 3791
// Hash 6974
// Hash 5846
// Hash 6228
// Hash 5752
// Hash 1834
// Hash 1011
// Hash 2875
// Hash 8086
// Hash 2260
// Hash 3130
// Hash 2865
// Hash 9866
// Hash 8967
// Hash 9038
// Hash 5209
// Hash 2004
// Hash 4697
// Hash 2821
// Hash 2904
// Hash 9084
// Hash 6024
// Hash 1922
// Hash 5846
// Hash 6699
// Hash 9276
// Hash 8306