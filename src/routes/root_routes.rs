pub fn init_root_routes(app: &mut tide::Server<()>) {
    app.at("/").get(|_| async { Ok(tide::Response::builder(tide::StatusCode::Ok).body("Hello, World!").build()) });
} 
