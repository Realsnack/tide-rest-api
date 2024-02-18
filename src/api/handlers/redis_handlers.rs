use crate::AppState;

pub async fn handle_get_key_value(req: tide::Request<AppState>) -> tide::Result<tide::Body> {
    let key = req.param("key").unwrap_or("Error");
    let redis = &req.state().redis;
    let redis_value = redis.get_key(key);

    // Ok(tide::Response::builder(tide::StatusCode::Ok).body(tide::Body::from_json(&redis_value)).build())
    Ok(tide::Body::from_json(&redis_value).unwrap())
}
