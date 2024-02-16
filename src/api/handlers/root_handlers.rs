pub async fn handle_get(mut req: tide::Request<()>) -> tide::Result {
    Ok(tide::Response::builder(tide::StatusCode::Ok).body("Hello, World!").build())
}

pub async fn handle_get_with_name(mut req: tide::Request<()>) -> tide::Result {
    let name = req.param("name").unwrap_or("Error");
    Ok(tide::Response::builder(tide::StatusCode::Ok).body(format!("Hello, {}!", name)).build())
}
