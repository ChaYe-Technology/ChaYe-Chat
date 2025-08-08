use dioxus::prelude::*;
use reqwest::Client;
use serde_json::{Value, json};
use std::fs;
use futures::StreamExt;

#[component]
pub fn gui_openai() -> Element {
    let data = fs::read_to_string("assets/api_key.json")?;
    let json_data: Value = serde_json::from_str(&data)?;
    let openai_api_key = json_data["OpenAI"].as_str().unwrap().to_string();
    let openai_model = "o4-mini-2025-04-16";
    let openai_path = "assets/convo/openai/convo2.json";

    let mut prompt = use_signal(|| String::new());
    let mut response = use_signal(|| None as Option<String>);
    rsx!{
        div { "OpenAI" }
        div {  
            input {  
                type: "text",
                oninput: move |event| prompt.set(event.value().to_string())
            }

            button {
                onclick: move |_| {
                    response.set(None);
                    let prompt = prompt().clone();
                    let api_key = openai_api_key.clone();
                    spawn({
                        let mut response = response.clone();
                        async move {
                            match call_openai(response, &api_key, openai_model, &prompt, openai_path, true).await {
                                Ok(_) => (),
                                Err(res) => response.set(Some(format!("Error: {}", res)))
                            }
                        }
                    });
                },
                "Go"
            }

            if let Some(msg) = response() {
                div { "{msg}" }
            }
        }
    }
}

pub async fn call_openai(mut response: Signal<Option<String>>, api_key: &str, model: &str, prompt: &str, path: &str, new: bool) -> Result<String, String> {
    // Clear response
    response.set(Some(String::new()));

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

    // Stream response
    let mut stream = res.bytes_stream();

    while let Some(stream_chunk) = stream.next().await {
        // Separate chunk into lines
        let chunk = stream_chunk
            .map_err(|e| format!("Failed to read chunk: {}", e))?;
        let lines = String::from_utf8_lossy(&chunk);

        for line in lines.lines() {
            if let Some(data) = line.strip_prefix("data: ") {

                // Check if LLM response finished
                if data.trim() == "[DONE]" {

                    // Save LLM response
                    let response_opt_str = response.read();
                    if let Some(response_str) = &*response_opt_str {
                        json_modify_openai(path, "assistant", &response_str)?;
                        return Ok("Done".to_string());
                    } else {
                        return Err("No response".to_string());
                    }
                }

                // Get response from data
                let data_json: serde_json::Value = match serde_json::from_str(data) {
                    Ok(d) => d,
                    Err(_) => continue,
                };

                if let Some(new_response) = data_json["choices"][0]["delta"]["content"].as_str() {
                    if let Some(response_str) = response.write().as_mut() {
                        response_str.push_str(new_response);
                    }
                }
            }
        }
    }

    Ok("Done".to_string())
}

fn json_create_openai(path: &str, model: &str, prompt: &str) -> Result<(), String>{
    let json_data = json!({
        "model": model,
        "stream" : true,
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
