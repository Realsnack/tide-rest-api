use crate::models::redis_key::RedisKey;

use r2d2_redis::redis::Commands;
use tide::log;

pub struct RedisService {
    connection_pool: r2d2::Pool<r2d2_redis::RedisConnectionManager>,
}

impl RedisService {
    pub fn new(host: String, port: u16) -> RedisService {
        let connection_manager = r2d2_redis::RedisConnectionManager::new(format!("redis://{}:{}", host, port)).unwrap();

        let connection_pool = r2d2::Pool::builder()
            .max_size(16)
            .build(connection_manager)
            .unwrap();

        RedisService { connection_pool }
    }
    
    pub fn get_key(&self, key: &str) -> RedisKey {
        log::debug!("Getting key: {}", key);
        let mut conn = self.connection_pool.get().unwrap();
        let value: Option<String> = conn.get(key).unwrap();

        RedisKey::new(key.to_string(), value, None)
    }

    pub fn get_key_expiration(&self, key: &str) -> Option<i64> {
        log::debug!("Getting key expiration: {}", key);
        let mut conn = self.connection_pool.get().unwrap();
        let expiration: Option<i64> = conn.ttl(key).unwrap();
        
        expiration
    }

    pub fn set_key(&self, key: &str, value: &str) {
        log::debug!("Setting key: {} with value: {}", key, value);
        let mut conn = self.connection_pool.get().unwrap();
        let _: () = conn.set(key, value).unwrap();
    }

    pub fn set_redis_key(&self, redis_key: RedisKey) {
        log::debug!("Setting key: {} with value: {:?}", redis_key.key, redis_key.value);
        let mut conn = self.connection_pool.get().unwrap();
        let _: () = conn.set(redis_key.key, redis_key.value.unwrap()).unwrap();
    }

    pub fn get_key_string(&self, key: &str) -> String {
        let mut conn = self.connection_pool.get().unwrap();
        let value: String = conn.get(key).unwrap();

        value
    }

    pub fn set_key_string(&self, key: &str, value: &str) {
        let mut conn = self.connection_pool.get().unwrap();
        let _: () = conn.set(key, value).unwrap();
    }
}

