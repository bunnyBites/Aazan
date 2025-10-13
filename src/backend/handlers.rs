use crate::{database::sessions::create_session, models::session::CreateSession};
use axum::{
    extract::{Multipart, Path, State},
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

pub async fn upload_session_handler(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        /*
         * axum::extract::Multipart: This extractor streams the incoming request body, parsing it as a multipart form.
         * multipart.next_field(): We loop through each "part" of the form data. A form can contain multiple fields (e.g., a file and some text metadata).
         * .bytes().await: This consumes the data from the field and loads it into memory. For very large files, a streaming approach would be better, but for our 10MB limit, this is fine.
         * */
        if field.name() == Some("pdf_file") {
            let file_name = field.file_name().unwrap_or("unknown_file").to_string();
            let data = field.bytes().await.unwrap();

            tracing::info!("Received file '{}' with {} bytes", file_name, data.len());

            // extract text from the in-memory bytes
            match pdf_extract::extract_text_from_mem(&data) {
                Ok(text) => {
                    tracing::info!("Successfully extracted text from PDF.");
                    // for now, we'll just print the first 200 characters
                    let preview: String = text.chars().take(200).collect();
                    tracing::debug!("Extracted text preview: {}...", preview);
                }
                Err(e) => {
                    tracing::error!("Failed to extract text from PDF: {}", e);
                }
            }
            // break after processing the first valid file field
            break;
        }
    }
}
