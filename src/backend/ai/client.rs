use std::env;

use reqwest::Client;

use crate::backend::ai::{
    model::{Content, GeminiRequest, GeminiResponse, Part},
    prompt::CREATE_BODHI_PROMPT,
};

pub async fn call_gemini_api(
    study_material: String,
    mut conversation_history: Vec<Content>,
) -> Result<String, anyhow::Error> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let api_url =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent";

    let client = Client::new();

    let system_prompt = CREATE_BODHI_PROMPT.replace("{}", &study_material);

    // combine the system prompt with the actual user/model conversation
    let mut full_prompt_contents = vec![
        Content {
            role: "user".into(),
            parts: vec![Part {
                text: system_prompt,
            }],
        },
        Content {
            role: "model".into(),
            parts: vec![Part {
                text: "I'm ready to learn! Let's begin.".into(),
            }],
        },
    ];

    full_prompt_contents.append(&mut conversation_history);

    let payload = GeminiRequest {
        contents: full_prompt_contents,
    };

    // send the request and get the response
    let response = client
        .post(api_url)
        .query(&[("key", &api_key)])
        .json(&payload)
        .send()
        .await?
        .json::<GeminiResponse>()
        .await?;

    // extract the text from the response
    let bot_response_text = response
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.clone())
        .unwrap_or_else(|| "I'm sorry, I'm not sure how to respond to that.".to_string());

    Ok(bot_response_text)
}
