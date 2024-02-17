mod api;
mod routes;
mod services;

use routes::root_routes::init_root_routes;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    init_root_routes(&mut app);

    // services
    let redis = crate::services::redis_service::RedisService::new("127.0.0.1".to_string(), 6379);

    println!("Listening on port 8080");
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

