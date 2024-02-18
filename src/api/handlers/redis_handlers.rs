use crate::AppState;

pub async fn handle_get_key_value(req: tide::Request<AppState>) -> tide::Result {
    let key = req.param("key").unwrap_or("Error");
    println!("Key: {}", key);
    let redis = &req.state().redis;
    let redis_value = redis.get_key(key);

    Ok(tide::Response::builder(tide::StatusCode::Ok).body(redis_value).build())
}
