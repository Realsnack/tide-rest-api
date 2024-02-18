mod api;
mod routes;
mod services;

use std::sync::Arc;

use routes::root_routes::init_root_routes;

use crate::{services::redis_service::RedisService, routes::redis_routes::init_redis_routes};

#[tokio::main]
async fn main() -> tide::Result<()> {
    // services
    let redis = RedisService::new("127.0.0.1".to_string(), 6379);
    let state = AppState { 
        redis: Arc::new(redis), 
    };

    let mut app = tide::with_state(state);

    init_root_routes(&mut app);
    init_redis_routes(&mut app);

    println!("Listening on port 8080");
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    redis: Arc<RedisService>,
}
