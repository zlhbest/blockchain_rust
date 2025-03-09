use rocksdb::DB;
use utils::coder;

use crate::block::Block;
const DB_DIR: &str = "./DB_DIR";
const BLOCKCHAIN_TARGET: &'static [u8; 1] = b"l";

/// 区块链的实现
pub struct BlockChain {
    /// 是DB中存储的最后一个区块
    pub tip: String,
    /// 遍历的时候用到
    current_hash: String,
    /// 数据库链接
    db: DB,
}

///todo 以下所有的数据更新操作应该使用事务来处理防止出错，但是目前时间有点紧，先实现功能再说
impl BlockChain {
    pub fn new() -> Self {
        // 第一步是检查 open_default该方法是如果没有数据库就创建
        let db = DB::open_default(DB_DIR).unwrap();
        // 查看该数据库中是否存储了相关数据
        let tip = match db.get(BLOCKCHAIN_TARGET).unwrap() {
            None => {
                //如果没有l, 那就说明这个里面不存在数据，那我就创建
                let genesis = Self::new_genesis_block();
                // 将hash存下来
                let _ = db.put(genesis.hash.clone(), coder::serialize_to_bincode(&genesis));
                // 将l存下来
                let _ = db.put(BLOCKCHAIN_TARGET, genesis.hash.clone());
                genesis.hash
            }
            Some(value) => {
                // 有的话就直接取出来,
                String::from_utf8(value).unwrap()
            }
        };
        Self {
            tip: tip.clone(),
            db,
            current_hash: tip,
        }
    }
    /// 创建创世区块
    fn new_genesis_block() -> Block {
        Block::new("This is genesis block".to_string(), String::default())
    }

    pub fn add(&mut self, data: String) {
        let new_block = Block::new(data, self.tip.clone());
        // 更新数据
        self.db
            .put(
                new_block.hash.clone(),
                coder::serialize_to_bincode(&new_block),
            )
            .unwrap();
        self.db
            .put(BLOCKCHAIN_TARGET, new_block.hash.clone())
            .unwrap();
        // 然后处理数据
        self.tip = new_block.hash.clone();
        // 指向最新的
        self.current_hash = new_block.hash.clone();
    }
}
/// 调用的into_iter
impl<'a> Iterator for &'a mut BlockChain {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        // 实现的迭代器
        match self.db.get(self.current_hash.as_bytes()) {
            Ok(raw_value) => match raw_value {
                None => None,
                Some(value) => {
                    // 反序列化
                    let block: Block = coder::bincode_deserialize(&value[..]);
                    self.current_hash = block.header.pre_hash.clone();
                    Some(block)
                }
            },
            Err(_) => None,
        }
    }
}
