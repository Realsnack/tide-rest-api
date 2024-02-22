use crate::AppState;

pub fn init_redis_routes(app: &mut tide::Server<AppState>) {
    let state = app.state().clone();

    app.at("/api/redis").nest({
        let mut api = tide::Server::with_state(state);

        api.at("/:key").get(crate::api::handlers::redis_handlers::handle_get_redis_key);
        api.at("/").post(crate::api::handlers::redis_handlers::handle_post_redis_key);

        api
    });
}

