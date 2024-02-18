pub mod redis_key;
#[derive(Clone)]

struct RedisKey {
    key: String,
    value: String,
    expiration: u32
}
