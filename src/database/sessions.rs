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

    let session = Session::from_strings(
        row.id,
        row.topic,
        row.material_text,
        row.status,
        row.created_at,
        row.updated_at,
        row.user_id.unwrap(),
    )
    .unwrap();

    Ok(session)
}

pub async fn get_session(pool: &SqlitePool, id: Uuid) -> Result<Session, sqlx::Error> {
    let id_str = id.to_string();

    let row = sqlx::query!(
        r#"
      SELECT * FROM sessions
      WHERE id = $1
      "#,
        id_str,
    )
    .fetch_one(pool)
    .await?;

    let session = Session::from_strings(
        row.id,
        row.topic,
        row.material_text,
        row.status,
        row.created_at,
        row.updated_at,
        row.user_id.unwrap(),
    )
    .unwrap();

    Ok(session)
}
