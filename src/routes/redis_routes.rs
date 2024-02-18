use crate::AppState;

pub fn init_redis_routes(app: &mut tide::Server<AppState>) {
    app.at("/api/redis/:key").get(crate::api::handlers::redis_handlers::handle_get_redis_key);
    app.at("/api/redis").post(crate::api::handlers::redis_handlers::handle_post_redis_key);
}
