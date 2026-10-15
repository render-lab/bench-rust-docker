use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn root() -> Json<Value> {
    Json(json!({ "message": "Seed Rust service. Add routes, then ship it." }))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let app = Router::new().route("/", get(root)).route("/health", get(health));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("listening on {port}");
    axum::serve(listener, app).await.unwrap();
}
