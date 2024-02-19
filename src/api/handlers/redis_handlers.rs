use crate::{AppState, models::redis_key::RedisKey};

pub async fn handle_get_redis_key(req: tide::Request<AppState>) -> tide::Result {
    let key = req.param("key").unwrap_or("Error");
    let redis = &req.state().redis;
    let mut redis_key = redis.get_key(key);
    let key_expiration = redis.get_key_expiration(key);
    redis_key.set_expiration(key_expiration);
    tide::log::debug!("Key: {} has value: {:?}", key, redis_key);

    if redis_key.value.is_none() {
        return Ok(tide::Response::builder(tide::StatusCode::NotFound).body(tide::Body::from_json(&redis_key).unwrap()).build());
    }

    Ok(tide::Response::builder(tide::StatusCode::Ok).body(tide::Body::from_json(&redis_key).unwrap()).build())
}

pub async fn handle_post_redis_key(mut req: tide::Request<AppState>) -> tide::Result {
    let body: RedisKey = req.body_json().await?;
    let redis = &req.state().redis;
    redis.set_redis_key(body);

    Ok(tide::Response::builder(tide::StatusCode::Ok).build())
}

