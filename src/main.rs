mod api;
mod routes;
mod services;

use routes::root_routes::init_root_routes;

use crate::services::redis_service::RedisService;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    init_root_routes(&mut app);

    // services
    let redis = RedisService::new("127.0.0.1".to_string(), 6379);
   
    redis.set_key("foo", "bar");
    let redis_key = redis.get_key("foo");
    println!("{}", redis_key);

    println!("Listening on port 8080");
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

