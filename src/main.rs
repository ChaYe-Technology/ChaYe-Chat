use dioxus::prelude::*;

mod api_openai;
mod api_anthropic;
mod api_deepseek;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut llm_select = use_signal(|| None as Option<&str>);

    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            background_color: "lightblue",
            padding: "10px",

            div {
                display: "flex",
                flex_direction: "column",
                background_color: "lightgreen",
                padding: "10px",

                button {
                    padding: "10px",

                    onclick: move |_| {
                        llm_select.set(Some("OpenAI"));
                    },
                    "OpenAI"
                }
                button {
                    padding: "10px",

                    onclick: move |_| {
                        llm_select.set(Some("Deepseek"));
                    },
                    "Deepseek"
                }
                button {
                    padding: "10px",

                    onclick: move |_| {
                        llm_select.set(Some("Anthropic"));
                    },
                    "Anthropic"
                }
            }
            div {
                background_color: "lightyellow",
                padding: "10px",

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
}
