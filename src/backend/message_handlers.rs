use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::message::CreateMessage;

pub async fn create_message_handler(
    State(pool): State<SqlitePool>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<CreateMessage>,
) -> impl IntoResponse {
    match crate::database::messages::create_message(&pool, session_id, payload).await {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// New handler to list messages for a session
pub async fn list_messages_handler(
    State(pool): State<SqlitePool>,
    Path(session_id): Path<Uuid>,
) -> impl IntoResponse {
    match crate::database::messages::list_messages_for_session(&pool, session_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list messages: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
