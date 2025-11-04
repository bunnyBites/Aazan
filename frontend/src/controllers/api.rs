use uuid::Uuid;

use crate::models::api::{Message, Session};

pub async fn get_messages(session_id: Uuid) -> Result<Vec<Message>, reqwest::Error> {
    let url = format!("http://localhost:3000/api/sessions/{}/messages", session_id);
    let messages = reqwest::get(&url).await?.json::<Vec<Message>>().await?;
    Ok(messages)
}

pub async fn list_sessions() -> Result<Vec<Session>, reqwest::Error> {
    let url = "http://localhost:3000/api/sessions";
    let sessions = reqwest::get(url).await?.json::<Vec<Session>>().await?;
    Ok(sessions)
}
