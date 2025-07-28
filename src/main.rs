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
    let api_key = "API-Key";
    let model = "gpt-4.1-2025-04-14";

    let mut response = use_signal(|| None as Option<String>);
    let mut prompt = use_signal(|| String::new());
    rsx! {
        div {
            input {  
                type: "text",
                oninput: move |event| prompt.set(event.value().to_string())
            }
            button {
                onclick: move |_| {
                    response.set(None);
                    let prompt = prompt().clone();
                    spawn({
                        let mut response = response.clone();
                        async move {
                            match call_openai(api_key, model, &prompt).await {
                                Ok(res) => response.set(Some(res)),
                                Err(res) => response.set(Some(format!("Error: {}", res)))
                            }
                        }
                    });
                },
                "Test OpenAI"
            }
            
            if let Some(msg) = response() {
                div { "{msg}" }
            }
        }
    }
}

async fn call_openai(api_key: &str, model: &str, prompt: &str) -> Result<String, String> {
    let client = Client::new();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "user", "content": prompt}
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
