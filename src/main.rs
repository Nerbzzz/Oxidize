use std::net::SocketAddr;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use tracing::info;
use tracing_subscriber;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "status": u16::from(StatusCode::OK),
        "message": "OK"
    })))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); 
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Starting Oxidize server on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}