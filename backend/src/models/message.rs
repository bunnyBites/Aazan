use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;
use uuid::Uuid;

// represents the two possible roles in a conversation
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl FromStr for MessageRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "User" => Ok(MessageRole::User),
            "Assistant" => Ok(MessageRole::Assistant),
            _ => Err(format!("Invalid MessageRole: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    #[serde(with = "uuid::serde::urn")]
    pub id: Uuid,
    #[serde(with = "uuid::serde::urn")]
    pub session_id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    /// Helper method to convert query row data into a Message struct
    pub fn from_query_row(
        id: Option<String>,
        session_id: String,
        role: String,
        content: String,
        timestamp: String,
    ) -> Result<Self, sqlx::Error> {
        Ok(Message {
            id: Uuid::parse_str(&id.unwrap()).map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            session_id: Uuid::parse_str(&session_id)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            role: role.parse().map_err(|e: String| {
                sqlx::Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e,
                )))
            })?,
            content,
            timestamp: timestamp
                .parse::<DateTime<Utc>>()
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
        })
    }
}

// represents the data we expect from the client to post a new message
#[derive(Debug, Deserialize)]
pub struct CreateMessage {
    pub role: MessageRole,
    pub content: String,
}
