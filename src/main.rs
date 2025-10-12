use axum::{Router, response::Html, routing::get};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting Aazan...");

    // Simple route
    let app = Router::new()
        .route("/", get(home_page))
        .route("/health", get(health_check));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("📡 Aazan server running at http://{}", addr);
    println!("📡 Aazan server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Server failed");
}

async fn home_page() -> Html<&'static str> {
    Html("<h1>Welcome to Aazan! 🎓</h1><p>Learn by Teaching - Backend is working!</p>")
}

async fn health_check() -> &'static str {
    "OK"
}
