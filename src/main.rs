use dioxus::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let api_key = "API-KEY";
    let model = "gpt-4.1-2025-04-14";

    let mut message = use_signal(|| None as Option<String>);
    rsx! {
        div {
            button {
                onclick: move |_| {
                    message.set(None);
                    spawn({
                        let mut message = message.clone();
                        async move {
                            match call_openai(api_key, model).await {
                                Ok(msg) => message.set(Some(msg)),
                                Err(msg) => message.set(Some(format!("Error: {}", msg))),
                            }
                        }
                    });
                },
                "Test OpenAI"
            }
            
            if let Some(msg) = message.read().as_ref() {
                div { "{msg}" }
            }
        }
    }
}

async fn call_openai(api_key: &str, model: &str) -> Result<String, String> {
    let client = Client::new();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "user", "content": "Hi, what is 1 + 1?"}
            ],
            "max_tokens": 500
        }))
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let api_response: ApiResponse = res
        .json()
        .await
        .map_err(|error| error.to_string())?;

    api_response
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| "No Rsponse".to_string())
}
