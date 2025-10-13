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

    let created_session = sqlx::query!(
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
        created_session.id,
        created_session.topic,
        created_session.material_text,
        created_session.status,
        created_session.created_at,
        created_session.updated_at,
        created_session.user_id.unwrap(),
    )
    .unwrap();

    Ok(session)
}

pub async fn get_session(pool: &SqlitePool, id: Uuid) -> Result<Session, sqlx::Error> {
    let id_str = id.to_string();

    let fetched_session = sqlx::query!(
        r#"
      SELECT * FROM sessions
      WHERE id = $1
      "#,
        id_str,
    )
    .fetch_one(pool)
    .await?;

    let session = Session::from_strings(
        fetched_session.id,
        fetched_session.topic,
        fetched_session.material_text,
        fetched_session.status,
        fetched_session.created_at,
        fetched_session.updated_at,
        fetched_session.user_id.unwrap(),
    )
    .unwrap();

    Ok(session)
}

pub async fn list_sessions(pool: &SqlitePool) -> Result<Vec<Session>, sqlx::Error> {
    let fetched_sessions = sqlx::query!(
        r#"
        SELECT * FROM sessions
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    let sessions: Result<Vec<Session>, _> = fetched_sessions
        .into_iter()
        .map(|fetched_session| {
            Session::from_strings(
                fetched_session.id,
                fetched_session.topic,
                fetched_session.material_text,
                fetched_session.status,
                fetched_session.created_at,
                fetched_session.updated_at,
                fetched_session.user_id.unwrap(),
            )
        })
        .collect();

    let sessions = sessions.map_err(|_| sqlx::Error::RowNotFound)?;
    Ok(sessions)
}
