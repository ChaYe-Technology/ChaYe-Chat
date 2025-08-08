use dioxus::prelude::*;
use reqwest::Client;
use serde_json::json;

#[component]
pub fn gui_anthropic() -> Element {
    rsx!{
        div { "Anthropic" }
    }
}

pub async fn call_anthropic(api_key: &str, model: &str, prompt: &str) -> Result<String, String> {
    let client = Client::new();

    let res = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "max_tokens": 5000
        }))
        .send()
        .await
        .map_err(|error| error.to_string())?;
    
    let raw_json = res.text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(raw_json)
}