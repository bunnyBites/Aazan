use crate::backend::handlers::{create_session_handler, get_session_handler};
use axum::{
    Router,
    response::Html,
    routing::{get, post},
};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

mod backend;
mod database;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Standard logging setup
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aazan=debug,tower_http=debug".into()),
        )
        .init();

    // Connect to the SQLite database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connection pool created.");

    let app = Router::new()
        .route("/api/sessions", post(create_session_handler))
        .route("/api/sessions/{:id}", get(get_session_handler))
        .route("/", get(|| home_page()))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn home_page() -> Html<&'static str> {
    Html("<h1>Welcome to Aazan! 🎓</h1><p>Learn by Teaching - Backend is working!</p>")
}
