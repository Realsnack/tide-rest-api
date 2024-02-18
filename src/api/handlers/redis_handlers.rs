use crate::AppState;

pub async fn handle_get_key_value(req: tide::Request<AppState>) -> tide::Result {
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

