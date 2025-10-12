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

impl Session {
    pub fn from_strings(
        id: String,
        topic: String,
        material_text: String,
        status: String,
        created_at: String,
        updated_at: String,
        user_id: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Session {
            id: Uuid::parse_str(&id)?,
            topic,
            material_text,
            status,
            created_at: created_at.parse()?,
            updated_at: updated_at.parse()?,
            user_id,
        })
    }
}

// represents the data we expect from the user to create a session
#[derive(Debug, Deserialize)]
pub struct CreateSession {
    pub topic: String,
    pub material_text: String,
}
