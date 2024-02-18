use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct RedisKey {
    pub key: String,
    pub value: Option<String>,
    pub expiration: Option<i64>,
}

impl RedisKey {
    pub fn new(key: String, value: Option<String>, expiration: Option<i64>) -> Self {
        RedisKey {
            key,
            value,
            expiration,
        }
    }

    pub fn set_expiration(&mut self, expiration: Option<i64>) {
        self.expiration = expiration;
    }
}
