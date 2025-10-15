use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    backend::ai::{
        client::call_gemini_api,
        model::{Content, Part},
    },
    database::sessions::get_session,
    models::{
        message::{CreateMessage, MessageRole},
        session::Session,
    },
};

pub async fn create_message_handler(
    State(pool): State<SqlitePool>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<CreateMessage>,
) -> impl IntoResponse {
    // save the user's message
    let user_message =
        match crate::database::messages::create_message(&pool, session_id, payload).await {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("Failed to save user message: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

    // fetch the full session context (material + history)
    let session: Session = match get_session(&pool, session_id).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to get session for AI call: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let history =
        match crate::database::messages::list_messages_for_session(&pool, session_id).await {
            Ok(h) => h,
            Err(e) => {
                tracing::error!("Failed to get history for AI call: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

    // call the Gemini API
    // convert our internal Message structs to the external AI model structs
    let conversation_history: Vec<Content> = history
        .into_iter()
        .map(|msg| Content {
            role: match msg.role {
                MessageRole::User => "user".to_string(),
                MessageRole::Assistant => "model".to_string(),
            },
            parts: vec![Part { text: msg.content }],
        })
        .collect();

    let ai_response_text = match call_gemini_api(session.material_text, conversation_history).await
    {
        Ok(text) => text,
        Err(e) => {
            tracing::error!("Gemini API call failed: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // save the AI's response
    let assistant_payload = CreateMessage {
        role: MessageRole::Assistant,
        content: ai_response_text,
    };

    let assistant_message =
        match crate::database::messages::create_message(&pool, session_id, assistant_payload).await
        {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("Failed to save assistant message: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

    // return both the user's and the assistant's messages
    (
        StatusCode::CREATED,
        Json(vec![user_message, assistant_message]),
    )
        .into_response()
}

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
