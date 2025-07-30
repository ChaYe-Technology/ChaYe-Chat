use dioxus::prelude::*;

mod api_openai;
mod api_anthropic;
mod api_deepseek;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let openai_api_key = "OpenAI API Key";
    let openai_model = "o4-mini-2025-04-16";
    let openai_path = "assets/convo/openai/convo2.json";

    let deepseek_api_key = "Deepseek API Key";
    let deepseek_model = "deepseek-reasoner";

    let anthropic_api_key = "Anthropic API Key";
    let anthropic_model = "claude-opus-4-20250514";

    let mut prompt = use_signal(|| String::new());
    let mut response = use_signal(|| None as Option<String>);

    rsx! {
        div {
            input {  
                type: "text",
                oninput: move |event| prompt.set(event.value().to_string())
            }
        }

        div {
            button {
                onclick: move |_| {
                    response.set(None);
                    let prompt = prompt().clone();
                    spawn({
                        let mut response = response.clone();
                        async move {
                            match api_openai::call_openai(openai_api_key, openai_model, &prompt, openai_path, true).await {
                                Ok(res) => response.set(Some(res)),
                                Err(res) => response.set(Some(format!("Error: {}", res)))
                            }
                        }
                    });
                },
                "Test OpenAI"
            }

            button {
                onclick: move |_| {
                    response.set(None);
                    let prompt = prompt().clone();
                    spawn({
                        let mut response = response.clone();
                        async move {
                            match api_deepseek::call_deepseek(deepseek_api_key, deepseek_model, &prompt).await {
                                Ok(res) => response.set(Some(res)),
                                Err(res) => response.set(Some(format!("Error: {}", res)))
                            }
                        }
                    });
                },
                "Test Deepseek"
            }

            button {
                onclick: move |_| {
                    response.set(None);
                    let prompt = prompt().clone();
                    spawn({
                        let mut response = response.clone();
                        async move {
                            match api_anthropic::call_anthropic(anthropic_api_key, anthropic_model, &prompt).await {
                                Ok(res) => response.set(Some(res)),
                                Err(res) => response.set(Some(format!("Error: {}", res)))
                            }
                        }
                    });
                },
                "Test Anthropic"
            }
            
            if let Some(msg) = response() {
                div { "{msg}" }
            }
        }
    }
}
