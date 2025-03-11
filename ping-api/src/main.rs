use axum::{routing::{get, any}, Router, response::{IntoResponse, Response}};
use std::net::SocketAddr;
use std::env;
use tokio::net::TcpListener;
use serde_json::json;
use axum::http::{Request, StatusCode};

// Handler pour GET /ping qui retourne les headers en JSON
async fn ping_handler(req: Request<axum::body::Body>) -> impl IntoResponse {
    let headers = req.headers().iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("invalid UTF-8").to_string()))
        .collect::<serde_json::Value>();
    
    Response::new(axum::body::Body::from(json!(headers).to_string()))
}

// Handler pour toutes les autres routes -> 404
async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "")
}

#[tokio::main]
async fn main() {
    // Charger les variables d'environnement
    dotenvy::dotenv().ok();
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/ping", any(not_found_handler)) // Capture tout sauf GET
        .fallback(not_found_handler);

    println!("🚀 Server listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
