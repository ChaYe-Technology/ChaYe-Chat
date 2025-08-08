use dioxus::{html::div, prelude::*};

mod api_openai;
mod api_anthropic;
mod api_deepseek;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {

    // let deepseek_api_key = "Deepseek API Key";
    // let deepseek_model = "deepseek-reasoner";

    // let anthropic_api_key = "Anthropic API Key";
    // let anthropic_model = "claude-opus-4-20250514";

    // let mut prompt = use_signal(|| String::new());
    // let mut response = use_signal(|| None as Option<String>);

    // let mut x = use_signal(|| 1);

    let mut llm_select = use_signal(|| None as Option<&str>);

    rsx! {
        div {
            button {
                onclick: move |_| {
                    llm_select.set(Some("OpenAI"));
                },
                "OpenAI"
            }
            button {
                onclick: move |_| {
                    llm_select.set(Some("Deepseek"));
                },
                "Deepseek"
            }
            button {
                onclick: move |_| {
                    llm_select.set(Some("Anthropic"));
                },
                "Anthropic"
            }
        }
        div {
            if llm_select() == None {
                "Hello"
            } else if llm_select() == Some("OpenAI") {
                api_openai::gui_openai {  }
            } else if llm_select() == Some("Deepseek") {
                api_deepseek::gui_deepseek {  }
            } else if llm_select() == Some("Anthropic") {
                api_anthropic::gui_anthropic {  }
            }
        }
    }
}
