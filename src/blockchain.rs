use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use time::UtcDateTime;

use crate::ui::{mining_spinner, print_block};

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub sender: String,
    pub reciever: String,
    pub amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    pub timestamp: i64,
    pub nonce: u32,
    pub pre_hash: String,
    pub merkle: String,
    pub difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    pub header: Blockheader,
    pub count: u32,
    pub transactions: Vec<Transaction>,
}

pub struct Chain {
    pub chain: Vec<Block>,
    pub curr_trans: Vec<Transaction>,
    pub difficulty: u32,
    pub miner_addr: String,
    pub reward: f32,
}

impl Chain {
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }

    pub fn new(miner_addr: String, difficulty: u32, reward: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: reward as f32,
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, reciever: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction {
            sender,
            reciever,
            amount,
        });
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: UtcDateTime::now().unix_timestamp(),
            nonce: 0,
            pre_hash: self.last_hash(),
            difficulty: self.difficulty,
            merkle: String::new(),
        };

        let reward_tx = Transaction {
            sender: String::from("root"),
            reciever: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_tx);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        let hash = Chain::hash(&block.header);
        print_block(&block, &hash);
        self.chain.push(block);
        true
    }

    fn get_merkle(curr_tx: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_tx {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        };

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        let pb = mining_spinner();

        loop {
            let hash = Chain::hash(header);

            if hash.starts_with(&"0".repeat(header.difficulty as usize)) {
                break;
            }

            header.nonce += 1;

            // update spinner text occasionally
            if header.nonce % 5_000 == 0 {
                pb.set_message(format!("Mining... nonce {}", header.nonce));
            }
        }
    }
}
