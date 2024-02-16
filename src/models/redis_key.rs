pub mod redis_key;

struct RedisKey {
    key: String,
    value: String,
    expiration: u32
}
