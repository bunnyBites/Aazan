use crate::{database::sessions::create_session, models::session::CreateSession};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::SqlitePool;
use uuid::Uuid;

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

pub async fn get_session_handler(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match crate::database::sessions::get_session(&pool, id).await {
        Ok(session) => (StatusCode::OK, Json(session)).into_response(),
        Err(sqlx::Error::RowNotFound) => {
            (StatusCode::NOT_FOUND, "Session not found").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get session: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve session",
            )
                .into_response()
        }
    }
}

pub async fn list_sessions_handler(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match crate::database::sessions::list_sessions(&pool).await {
        Ok(sessions) => (StatusCode::OK, Json(sessions)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list sessions: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve sessions",
            )
                .into_response()
        }
    }
}

pub async fn delete_session_handler(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match crate::database::sessions::delete_session(&pool, id).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND,
        Err(e) => {
            tracing::error!("Failed to delete session: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
