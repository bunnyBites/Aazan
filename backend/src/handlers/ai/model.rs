use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}

#[derive(Serialize)]
pub struct Content {
    pub role: String, // "user" or "model"
    pub parts: Vec<Part>,
}

#[derive(Serialize)]
pub struct Part {
    pub text: String,
}

// --- Gemini API Response Structures ---

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: ContentResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentResponse {
    pub parts: Vec<PartResponse>,
    pub _role: String,
}

#[derive(Deserialize)]
pub struct PartResponse {
    pub text: String,
}
