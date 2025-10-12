use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    #[serde(with = "uuid::serde::urn")]
    pub id: Uuid,
    pub topic: String,
    pub material_text: String,
    pub status: String, // we can make this an enum later
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: String, // We'll add this when auth is implemented
}

// represents the data we expect from the user to create a session
#[derive(Debug, Deserialize)]
pub struct CreateSession {
    pub topic: String,
    pub material_text: String,
}
