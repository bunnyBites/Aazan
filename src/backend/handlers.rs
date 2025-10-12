use crate::{database::sessions::create_session, models::session::CreateSession};
use axum::{extract::State, http::StatusCode, response::IntoResponse, response::Json};
use sqlx::SqlitePool;

pub async fn create_session_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateSession>,
) -> impl IntoResponse {
    match create_session(&pool, payload).await {
        Ok(session) => (StatusCode::CREATED, Json(session)).into_response(),
        Err(e) => {
            // It's good practice to log the error
            tracing::error!("Failed to create session: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create session",
            )
                .into_response()
        }
    }
}
