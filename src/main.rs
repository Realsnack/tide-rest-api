#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/").get(|_| async { Ok(tide::Response::builder(tide::StatusCode::Ok).body("Hello, World!").build()) } );

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

fn hello_world(_req: tide::Request<()>) -> tide::Result {
    let response = tide::Response::builder(tide::StatusCode::Ok)
        .body("Hello, World!")
        .build();

    Ok(response)
}
