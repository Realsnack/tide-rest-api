use r2d2_redis::redis::Commands;

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

    pub fn get_key(&self, key: &str) -> String {
        let mut conn = self.connection_pool.get().unwrap();
        let value: String = conn.get(key).unwrap();

        value
    }

    pub fn set_key(&self, key: &str, value: &str) {
        let mut conn = self.connection_pool.get().unwrap();
        let _: () = conn.set(key, value).unwrap();
    }
}

