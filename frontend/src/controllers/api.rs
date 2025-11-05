use uuid::Uuid;

use crate::models::api::{CreateSessionPayload, Message, Session};

pub async fn get_messages(session_id: Uuid) -> Result<Vec<Message>, reqwest::Error> {
    let url = format!("/api/sessions/{}/messages", session_id);
    let messages = reqwest::get(&url).await?.json::<Vec<Message>>().await?;
    Ok(messages)
}

pub async fn list_sessions() -> Result<Vec<Session>, reqwest::Error> {
    let url = "/api/sessions";
    let sessions = reqwest::get(url).await?.json::<Vec<Session>>().await?;
    Ok(sessions)
}

pub async fn create_session(
    topic: String,
    material_text: String,
) -> Result<Session, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "/api/sessions";

    let payload = CreateSessionPayload {
        topic,
        material_text,
    };

    let response = client
        .post(url)
        .json(&payload)
        .send()
        .await?
        .json::<Session>()
        .await?;

    Ok(response)
}
