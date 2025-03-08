use bincode;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

/// 将信息序列化成二进制code
pub fn serialize_to_bincode<T: ?Sized + Serialize>(value: &T) -> Vec<u8> {
    bincode::serialize(value).unwrap()
}
/// 将字节数组转为value
pub fn bincode_deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    bincode::deserialize(bytes).unwrap()
}

/// 求hash函数
pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[cfg(test)]
mod tests {

    use super::{bincode_deserialize, serialize_to_bincode};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct Pointer {
        x: i32,
        y: i32,
    }
    #[test]
    fn serialize_deserialize() {
        let point = Pointer { x: 1, y: 1 };
        // 序列化
        let se = serialize_to_bincode(&point);
        // 反序列化
        let de: Pointer = bincode_deserialize(&se);
        assert_eq!(point, de);
    }
}
