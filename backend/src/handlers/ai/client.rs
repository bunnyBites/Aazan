use std::env;

use futures_util::Stream;
use reqwest::Client;
use tokio_stream::StreamExt;

use crate::handlers::ai::{
    model::{Content, GeminiRequest, GeminiResponse, Part},
    prompt::CREATE_BODHI_PROMPT,
};

pub async fn call_gemini_api(
    study_material: String,
    mut conversation_history: Vec<Content>,
) -> Result<String, anyhow::Error> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let api_url =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

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

pub fn stream_gemini_api(
    study_material: String,
    mut conversation_history: Vec<Content>,
) -> impl Stream<Item = Result<String, anyhow::Error>> {
    async_stream::try_stream! {
        let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
        let api_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:streamGenerateContent";

        let client = Client::new();

        let system_prompt = CREATE_BODHI_PROMPT.replace("{}", &study_material);
        let mut full_prompt_contents = vec![
            Content { role: "user".into(), parts: vec![Part { text: system_prompt }] },
            Content { role: "model".into(), parts: vec![Part { text: "I'm ready to learn! Let's begin.".into() }] }
        ];
        full_prompt_contents.append(&mut conversation_history);

        let payload = GeminiRequest {
            contents: full_prompt_contents,
        };

        // get a stream of bytes from the response
        let mut byte_stream = client
            .post(api_url)
            .query(&[("key", &api_key)])
            .json(&payload)
            .send()
            .await?
            .bytes_stream();

        // process the stream
        while let Some(chunk) = byte_stream.next().await {
            let chunk = chunk?;
            // NOTE: This is a simplification. It assumes each chunk from the API is a self-contained, valid JSON object.
            // A more robust solution would buffer bytes and parse multiple JSON objects from a single chunk.
            if let Ok(response) = serde_json::from_slice::<GeminiResponse>(&chunk) {
                if let Some(text) = response.candidates.get(0).and_then(|c| c.content.parts.get(0)).map(|p| p.text.clone()) {
                    yield text;
                }
            }
        }
    }
}
