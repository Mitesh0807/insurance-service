use axum::http;
use axum::routing::{get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let app = Router::new().route("/", get(health_check));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}
