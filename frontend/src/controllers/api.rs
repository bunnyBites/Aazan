use uuid::Uuid;

use crate::models::api::Message;

// This is our "Controller" logic for fetching data.
pub async fn get_messages(session_id: Uuid) -> Result<Vec<Message>, reqwest::Error> {
    let url = format!("http://localhost:3000/api/sessions/{}/messages", session_id);
    let messages = reqwest::get(&url).await?.json::<Vec<Message>>().await?;
    Ok(messages)
}
