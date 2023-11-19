use axum::http;
use axum::routing::{get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on {}", addr);

    let app = Router::new().route("/", get(healt));

    axum::Server::bind(&addr.parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}

async fn healt() -> http::Response<String> {
    http::Response::new("Hello, World!".into())
}