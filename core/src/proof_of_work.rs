use std::u64::{self, MAX};

use num_bigint::{BigUint, ToBigUint};
use utils::coder;

use crate::block::Block;
const TARGET_BITS: u32 = 16;
/// 工作量证明的具体含义是找到一个合适的数字，通过hash 计算出一个结果，该结果的前几位保证是0
#[derive(Debug)]
pub struct ProofOfWork<'a> {
    pub block: &'a Block,
    pub target: BigUint,
}
impl<'a> ProofOfWork<'a> {
    pub fn new(block: &'a Block) -> Self {
        let mut target = 1.to_biguint().unwrap();
        // 计算左移的位数
        let shift_amount = 256 - TARGET_BITS;
        // 向左移动
        target <<= shift_amount;
        Self { block, target }
    }
    /// 该函数的功能就是将输入的数字与block头进行组合
    fn prepare_data(&self, nonce: u64) -> Vec<u8> {
        let mut data = coder::serialize_to_bincode(&self.block.header);
        // 然后将target_bits加进入
        data.append(&mut coder::serialize_to_bincode(&TARGET_BITS));
        // 将数组加进入
        data.append(&mut coder::serialize_to_bincode(&nonce));
        data
    }

    /// 实现计算算法，目的是找到一个数字，实现hash以后前几位是target_bits位是0
    pub fn run(&self) -> (u64, String) {
        let mut hash_int: BigUint;
        let mut hash = String::default();
        let mut nonce = 0;
        while nonce < MAX {
            let data = self.prepare_data(nonce);
            hash = coder::get_hash(&data[..]);
            hash_int = BigUint::parse_bytes(hash.as_bytes(), 16).unwrap();
            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, hash)
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let hash = coder::get_hash(&data);
        let hash_int = BigUint::parse_bytes(hash.as_bytes(), 16).unwrap();
        hash_int < self.target
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;

    use crate::block::Block;

    #[test]
    fn proof_of_work_new_test() {
        let mut target = 1.to_biguint().unwrap();
        println!("{:x}", target);
        // 计算左移的位数
        let shift_amount = 256 - 16;
        // 向左移动
        target <<= shift_amount;
        println!("{:x}", target);
    }

    #[test]
    fn prepare_data_test() {
        println!("开始");
        Block::new("hello".to_string(), "".to_string());
    }
}
