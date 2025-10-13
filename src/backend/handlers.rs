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

pub async fn upload_session_handler(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut topic: Option<String> = None;
    let mut material_text: Option<String> = None;

    // loop through all fields to find topic and the PDF file
    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some(name) = field.name() {
            match name {
                "topic" => {
                    if let Ok(data) = field.text().await {
                        topic = Some(data);
                    }
                }
                "pdf_file" => {
                    if let Ok(data) = field.bytes().await {
                        match pdf_extract::extract_text_from_mem(&data) {
                            Ok(text) => material_text = Some(text),
                            Err(e) => {
                                tracing::error!("PDF extraction failed: {}", e);
                                return (
                                    StatusCode::UNPROCESSABLE_ENTITY,
                                    "Invalid or corrupted PDF",
                                )
                                    .into_response();
                            }
                        }
                    }
                }
                _ => {} // ignore other fields
            }
        }
    }

    // validate that we have both a topic and extracted text
    if let (Some(topic), Some(material_text)) = (topic, material_text) {
        let payload = CreateSession {
            topic,
            material_text,
        };

        // call the existing create_session database function
        match crate::database::sessions::create_session(&pool, payload).await {
            Ok(session) => (StatusCode::CREATED, Json(session)).into_response(),
            Err(e) => {
                tracing::error!("Failed to create session from upload: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, "Missing 'topic' or 'pdf_file'").into_response()
    }
}
