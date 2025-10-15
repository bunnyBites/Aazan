use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
};
use futures_util::{
    StreamExt,
    stream::{self, Stream},
};
use sqlx::SqlitePool;
use std::convert::Infallible;
use uuid::Uuid;

use crate::{
    backend::ai::{
        client::stream_gemini_api,
        model::{Content, Part},
    },
    database::{messages, sessions},
    models::message::MessageRole,
};

pub async fn sse_handler(
    State(pool): State<SqlitePool>,
    Path(session_id): Path<Uuid>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // fetch the session first and handle the result properly
    let session_result = sessions::get_session(&pool, session_id).await;

    let stream = match session_result {
        // if we found the session, proceed to create the AI stream
        Ok(session) => {
            let history = messages::list_messages_for_session(&pool, session_id)
                .await
                .unwrap_or_else(|_| vec![]);

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

            let ai_stream = stream_gemini_api(session.material_text, conversation_history);

            // map the AI stream results into SSE Events
            ai_stream
                .map(|result| match result {
                    Ok(text) => Event::default().data(text),
                    Err(e) => {
                        tracing::error!("Stream error: {}", e);
                        Event::default()
                            .event("error")
                            .data("An error occurred during the stream.")
                    }
                })
                // add this crucial line to wrap the Event in a Result
                .map(Ok)
                .left_stream()
        }
        // if the session was not found, create a stream with a single error event
        Err(e) => {
            tracing::error!("Initial SSE connection failed, session not found: {}", e);
            let error_message = format!("Session with ID {} not found.", session_id);
            let error_stream =
                stream::once(async { Ok(Event::default().event("error").data(error_message)) });
            error_stream.right_stream()
        }
    };

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
