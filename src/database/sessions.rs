use crate::models::session::{CreateSession, Session};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_session(
    pool: &SqlitePool,
    new_session: CreateSession,
) -> Result<Session, sqlx::Error> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let id_str = id.to_string();
    let created_at_str = now.to_rfc3339();
    let updated_at_str = now.to_rfc3339();

    let row = sqlx::query!(
        r#"
        INSERT INTO sessions (id, topic, material_text, status, created_at, updated_at, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, topic, material_text, status, created_at, updated_at, user_id
        "#,
        id_str,
        new_session.topic,
        new_session.material_text,
        "created",
        created_at_str,
        updated_at_str,
        "temp_user" // placeholder user_id
    )
    .fetch_one(pool)
    .await?;

    let session = Session {
        id: Uuid::parse_str(&row.id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
        topic: row.topic,
        material_text: row.material_text,
        status: row.status,
        created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .with_timezone(&Utc),
        updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .with_timezone(&Utc),
        user_id: row.user_id.unwrap_or_else(|| "temp_user".to_string()),
    };

    Ok(session)
}
