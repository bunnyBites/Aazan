use crate::models::message::{CreateMessage, Message};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

/// inserts a new message into the database for a given session.
pub async fn create_message(
    pool: &SqlitePool,
    session_id: Uuid,
    new_message: CreateMessage,
) -> Result<Message, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let session_id_str = session_id.to_string();
    let role_str = format!("{:?}", new_message.role);
    let timestamp_str = Utc::now().to_rfc3339();

    let message = sqlx::query!(
        r#"
        INSERT INTO messages (id, session_id, role, content, timestamp)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, session_id, role, content, timestamp
        "#,
        id,
        session_id_str,
        role_str,
        new_message.content,
        timestamp_str
    )
    .fetch_one(pool)
    .await?;

    let result = Message::from_query_row(
        message.id,
        message.session_id,
        message.role,
        message.content,
        message.timestamp,
    )?;

    Ok(result)
}

/// lists all messages for a specific session, ordered by when they were created.
pub async fn list_messages_for_session(
    pool: &SqlitePool,
    session_id: Uuid,
) -> Result<Vec<Message>, sqlx::Error> {
    let session_id_str = session_id.to_string();

    let rows = sqlx::query!(
        r#"
        SELECT id, session_id, role, content, timestamp FROM messages
        WHERE session_id = $1
        ORDER BY timestamp ASC
        "#,
        session_id_str
    )
    .fetch_all(pool)
    .await?;

    let mut messages = Vec::new();
    for row in rows {
        let message =
            Message::from_query_row(row.id, row.session_id, row.role, row.content, row.timestamp)?;
        messages.push(message);
    }

    Ok(messages)
}
