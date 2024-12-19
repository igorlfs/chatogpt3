pub mod lib;

use lib::{Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part};
use reqwest::Client;

static BASEURL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent";

pub async fn get_chat_response(
    apikey: &str,
    history: Vec<Content>,
) -> (Option<String>, Option<String>) {
    let payload = GenerateContentRequest {
        contents: history,
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(100),
            temperature: Some(0.4),
            top_p: Some(1.0),
            top_k: Some(32),
            ..Default::default()
        }),
        tools: None,
    };

    match Client::new()
        .post(format!("{BASEURL}?key={apikey}"))
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => match response.json::<GenerateContentResponse>().await {
            Ok(gemini_response) => {
                let aux = &gemini_response.candidates[0].content.parts[0];
                match aux {
                    Part::Text(text) => (Some(text.clone()), None),
                    _ => (None, Some(String::from("Unexpected response from Gemini"))),
                }
            }
            Err(e) => (
                None,
                Some(format!(
                    "An error occurred while processing the response: {e}"
                )),
            ),
        },
        Err(e) => (
            None,
            Some(format!("An error occurred when making a request: {e}")),
        ),
    }
}
