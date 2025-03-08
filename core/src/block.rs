use chrono::Utc;
use serde::{Deserialize, Serialize};
use utils::coder;

use crate::proof_of_work::ProofOfWork;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    /// 时间戳
    pub time: i64,
    /// 对整个交易数据求hash
    pub tx_hash: String,
    /// 前一条链的hash
    pub pre_hash: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    /// 对整个头求hash
    pub hash: String,
    /// 存储的数据
    pub data: String,

    pub nonce: u64,
}

impl Block {
    fn set_hash(&mut self) {
        // set_hash的时候进行工作量证明
        let pow = ProofOfWork::new(&self);
        //hash 存的是头的hash
        let (nonce, hash) = pow.run();
        self.hash = hash;
        self.nonce = nonce;
    }

    pub fn new(data: String, pre_hash: String) -> Block {
        // 对数据取hash,按理tx_hash是按照特定方式实现的
        let transactions = coder::serialize_to_bincode(&data);
        // 是整个交易的hash
        let tx_hash = coder::get_hash(&transactions);
        let mut block = Block {
            header: BlockHeader {
                time: Utc::now().timestamp(),
                tx_hash,
                // 存上一个的hash
                pre_hash,
            },
            // 存的是整个头部的hash
            hash: String::default(),
            data,
            nonce: 0,
        };
        block.set_hash();
        block
    }
}
