use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct RedisKey {
    pub key: String,
    pub value: Option<String>,
    pub expiration: Option<u32>,
}

impl RedisKey {
    pub fn new(key: String, value: Option<String>, expiration: Option<u32>) -> Self {
        RedisKey {
            key,
            value,
            expiration,
        }
    }
}
