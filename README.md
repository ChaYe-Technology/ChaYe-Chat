# ChaYe Chat (Work in Progress)

### Purpose
A project aimed at creating a easy to use, and stylistically simple, LLM API interaction application that allows 
users to input their own API keys to use state of the art models without subscribing to a dozen different models
that costs upwards of $20 each.

### Reason
Having used LLMs extensively, I was personally ok with paying $20 for a single subscription. However, as each
model has different strengths and weaknesses, I didn't want to be stuck buying into a single company's
subscription. Having tried utilizing APIs and local models for other projects, the idea of a standalone AI app
that can interact with all potential state of the art models sounded appealing to me. Plus, having an app to 
replace the keybinding for my Copilot button on my laptop is another bonus.

### Tech Stack
This project utilizes the Rust language and Dioxus for the GUI. With no official crates for OpenAI, Anthropic,
Gemini, or Deepseek, this project would utilize Reqwest to use the APIs.
