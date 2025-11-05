use crate::handlers::{
    message_handlers::{create_message_handler, list_messages_handler},
    session_handlers::{
        create_session_handler, delete_session_handler, get_session_handler, list_sessions_handler,
        upload_session_handler,
    },
    stream_handlers::sse_handler,
};
use axum::{
    Router,
    http::HeaderValue,
    routing::{delete, get, post},
};
use reqwest::Method;
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

mod database;
mod handlers;
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

    // connect to the SQLite database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connection pool created.");

    // Get allowed origins from environment variable or use defaults
    let allowed_origins = std::env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| {
        "http://localhost:8081,http://127.0.0.1:8081,https://aazan-dry-pond-469.fly.dev".to_string()
    });

    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::DELETE])
        .allow_headers(Any);

    let routes = Router::new()
        .route("/api/sessions", get(list_sessions_handler))
        .route("/api/sessions", post(create_session_handler))
        .route("/api/sessions/upload", post(upload_session_handler))
        .route("/api/sessions/{:id}", get(get_session_handler))
        .route("/api/sessions/{:id}", delete(delete_session_handler))
        .route("/api/sessions/{:id}/stream", get(sse_handler))
        .route(
            "/api/sessions/{:id}/messages",
            get(list_messages_handler).post(create_message_handler),
        );

    let app = Router::new()
        .nest("/api", routes)
        // nested message routes
        .with_state(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .nest_service("/", ServeDir::new("dist"));

    // Run the server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port: u16 = port.parse().expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
