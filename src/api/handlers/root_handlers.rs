use crate::AppState;

pub async fn handle_get(_: tide::Request<AppState>) -> tide::Result {
    Ok(tide::Response::builder(tide::StatusCode::Ok).body("Hello, World!").build())
}

pub async fn handle_get_with_name(req: tide::Request<AppState>) -> tide::Result {
    let name = req.param("name").unwrap_or("Error");
    Ok(tide::Response::builder(tide::StatusCode::Ok).body(format!("Hello, {}!", name)).build())
}
