mod handlers;
use axum::routing::{get, post, Router};
use sqlx::postgres::PgPoolOptions;
use std::{env, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>>{
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;

    println!("Listening on {}", addr);

    let app = Router::new()
        .route("/", get(handlers::health))
        .route("/quotes", post(handlers::create_quote))
        .with_state(pool);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;
    

    Ok(())
}