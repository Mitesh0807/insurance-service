use axum::routing::{get, post, Router};
mod handlers;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| ("8080".to_string()));
    let addr = format!("0.0.0.0:{}", port);
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("Successfully connected to the database.");
    let app = Router::new()
        .route("/", get(handlers::health_check))
        .route("/customers", get(handlers::get_all_customers))
        .route("/customers", post(handlers::create_customer))
        .with_state(pool);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
