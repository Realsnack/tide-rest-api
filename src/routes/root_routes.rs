use crate::AppState;

pub fn init_root_routes(app: &mut tide::Server<AppState>) {
    app.at("/").get(crate::api::handlers::root_handlers::handle_get);
    app.at("/:name").get(crate::api::handlers::root_handlers::handle_get_with_name);
} 
