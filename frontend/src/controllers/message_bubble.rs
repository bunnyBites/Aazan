use uuid::Uuid;

use crate::models::api::{CreateMessage, Message, MessageRole};

pub async fn send_message(
    session_id: Uuid,
    content: String,
) -> Result<Vec<Message>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:3000/api/sessions/{}/messages", session_id);

    let payload = CreateMessage {
        role: MessageRole::User,
        content,
    };

    // backend returns an array with the user's message and the assistant's response
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?
        .json::<Vec<Message>>()
        .await?;
    Ok(response)
}
