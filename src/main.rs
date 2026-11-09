use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn root() -> &'static str {
    "Hello from Rust!"
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("failed to bind address");

    println!("Listening on 0.0.0.0:{port}");
    axum::serve(listener, app).await.expect("server error");
}
