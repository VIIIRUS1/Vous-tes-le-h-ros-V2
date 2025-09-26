use axum::{routing::get, Router};
use std::net::SocketAddr;

mod db;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    db::init().await.expect("Failed to initialize database");

    let app = Router::new()
        .route("/", get(|| async { "Hello from Axum!" }))
        .route("/users", get(routes::get_users));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
