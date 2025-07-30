use reqwest::Client;
use serde_json::{Value, json};
use std::fs;

pub async fn call_openai(api_key: &str, model: &str, prompt: &str, path: &str, new: bool) -> Result<String, String> {
    // Get conversation json
    if new {
        json_create_openai(path, model, prompt)?;
    } else {
        json_modify_openai(path, "user", prompt)?;
    }

    let data = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let convo: Value = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Create request
    let client = Client::new();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&convo)
        .send()
        .await
        .map_err(|error| error.to_string())?;
    
    let raw_json = res.text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let parsed: serde_json::Value = serde_json::from_str(&raw_json)
    .map_err(|e| format!("Failed to parse response JSON: {}", e))?;

    let content = parsed["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content found in response")?
        .to_string();

    json_modify_openai(path, "assistant", content.as_str())?;

    Ok(content)
}

fn json_create_openai(path: &str, model: &str, prompt: &str) -> Result<(), String>{
    let json_data = json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let serialized = serde_json::to_string_pretty(&json_data)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    fs::write(path, serialized)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

fn json_modify_openai(path: &str, role: &str, prompt: &str) -> Result<(), String> {
    let data = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut json_data: Value = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    if let Some(messages) = json_data["messages"].as_array_mut() {
        messages.push(json!({
            "role": role,
            "content": prompt
        }));
    } else {
        return Err("JSON does not contain a 'messages' list".into());
    }

    let serialized = serde_json::to_string_pretty(&json_data)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    fs::write(path, serialized)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}
