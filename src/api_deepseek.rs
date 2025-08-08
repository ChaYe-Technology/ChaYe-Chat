use dioxus::prelude::*;
use reqwest::Client;
use serde_json::json;

#[component]
pub fn gui_deepseek() -> Element {
    rsx!{
        div { "Deepseek" }
    }
}

pub async fn call_deepseek(api_key: &str, model: &str, prompt: &str) -> Result<String, String> {
    let client = Client::new();

    let res = client
        .post("https://api.deepseek.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        }))
        .send()
        .await
        .map_err(|error| error.to_string())?;
    
    let raw_json = res.text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(raw_json)
}