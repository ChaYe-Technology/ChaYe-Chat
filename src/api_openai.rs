use dioxus::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use futures::StreamExt;

#[derive(Deserialize)]
struct ConvoStruct {
    file: String,
    title: String,
    model: String,
    new: bool
}

#[derive(Deserialize)]
struct ConvoStructVec {
    conversations: Vec<ConvoStruct>
}

#[derive(Deserialize)]
struct MsgStruct {
    role: String,
    content: String
}

#[derive(Deserialize)]
struct MsgStructVec {
    messages: Vec<MsgStruct>
}

#[component]
pub fn gui_openai() -> Element {
    // Get API key
    let data = fs::read_to_string("assets/api_key.json")?;
    let json_data: Value = serde_json::from_str(&data)?;
    let openai_api_key = json_data["OpenAI"].as_str().unwrap().to_string();

    // Models
    let openai_model_lis = vec![
        "gpt-5-nano-2025-08-07", 
        "gpt-5-2025-08-07",
        "o4-mini-2025-04-16",
        "o3-2025-04-16"
    ];
    let mut openai_model = use_signal(|| openai_model_lis[0]);

    let mut input = use_signal(|| String::new());
    let mut response = use_signal(|| None as Option<String>);
    let mut streaming = use_signal(|| false); 

    let convo_lis = table_openai();
    let mut convo_idx = use_signal(|| 0);

    let convo_cur: Vec<MsgStruct> = if convo_lis[convo_idx()].new == false {
        let path = convo_lis[convo_idx()].file.clone();
        let data = fs::read_to_string(path)?;
        let json_data: MsgStructVec = serde_json::from_str(&data)?;
        json_data.messages
    } else {
        Vec::new()
    };

    rsx!{
        div {
            display: "flex",
            flex_direction: "column",
            background_color: "lightgreen",
            padding: "10px",

            // Header and Model Selector
            div {
                background_color: "lightblue",
                padding: "10px",
                div { "OpenAI" }
            }

            div {
                display: "flex",
                flex_direction: "row",
                background_color: "lightyellow",
                padding: "10px",

                // Conversation List
                div {
                    background_color: "lightblue",
                    padding: "10px",

                    for i in 0..convo_lis.len() {
                        div {
                            button { 
                                onclick: move |_| {
                                    convo_idx.set(i);
                                },
                                span {
                                    display: "block",
                                    "{convo_lis[i].title}"
                                },
                                span {
                                    display: "block",
                                    "{convo_lis[i].model}"
                                },
                            }
                        }
                    }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    background_color: "lightgreen",
                    padding: "10px",

                    // Current Conversation
                    div {
                        background_color: "lightyellow",
                        padding: "10px",

                        if convo_lis[convo_idx()].new == false {
                            for x in convo_cur {
                                if x.role == "assistant" {
                                    div {
                                        background_color: "lightpink",
                                        padding: "10px",

                                        div {
                                            "OpenAI"
                                        }
                                        div {  
                                            padding_left: "20px",
                                            "{x.content}"
                                        }
                                    }
                                } else {
                                    div {
                                        background_color: "lightblue",
                                        padding: "10px",

                                        div {
                                            "You"
                                        }
                                        div {  
                                            padding_left: "20px",
                                            "{x.content}"
                                        }
                                    }
                                }
                            }
                        }
                        if streaming() {
                            if let Some(msg) = response() {
                                div { 
                                    background_color: "lightgreen",
                                    padding: "10px",

                                    div {
                                        "Openai"
                                    }
                                    div {
                                        padding_left: "20px",
                                        "{msg}" 
                                    }
                                }
                            }
                        }
                    }

                    // Prompt Box
                    div {  
                        background_color: "lightpink",
                        padding: "10px",

                        input {  
                            type: "text",
                            value: "{input}",
                            oninput: move |event| input.set(event.value().to_string())
                        }

                        button {
                            onclick: move |_| {
                                response.set(None);
                                streaming.set(true);
                                let prompt = input().clone();
                                let api_key = openai_api_key.clone();
                                let openai_model = openai_model();
                                let openai_path = convo_lis[convo_idx()].file.clone();
                                let new = convo_lis[convo_idx()].new.clone();
                                input.set(String::new());
                                spawn({
                                    let mut response = response.clone();
                                    async move {
                                        match call_openai(response, &api_key, openai_model, &prompt, openai_path.as_str(), new, streaming).await {
                                            Ok(_) => (),
                                            Err(res) => response.set(Some(format!("Error: {}", res)))
                                        }
                                    }
                                });
                            },
                            "Go"
                        }

                        div { "{openai_model}" }
                        div {
                            for model in openai_model_lis {
                                button {
                                    onclick: move |_| {
                                        openai_model.set(model);
                                    },
                                    "{model}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn call_openai(mut response: Signal<Option<String>>, api_key: &str, model: &str, prompt: &str, path: &str, new: bool, mut streaming: Signal<bool>) -> Result<String, String> {
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
                    streaming.set(false);

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

fn table_openai() -> Vec<ConvoStruct> {
    let data = fs::read_to_string("assets/convo/openai/openai_table.json")
        .map_err(|e| format!("Failed to read file: {}", e)).unwrap();

    let json_data: ConvoStructVec = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e)).unwrap();

    return json_data.conversations
}
