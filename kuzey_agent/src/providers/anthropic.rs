use super::{ProviderConfig, ChatMessage, Reply};
use crate::tools::ToolSpec;
use serde_json::{json, Value};

//Still need to integrate the tools. For an example check ollama.rs"
pub(super) async fn send(config: &ProviderConfig, history: &[ChatMessage], tools: &[ToolSpec]) -> Result<Reply, Box<dyn std::error::Error>> {

    let api_key = config.api_key.as_deref().ok_or("Anthropic needs an api key!")?;

    let mut messages: Vec<Value> = Vec::new();

    for entry in history {
        match entry {
            ChatMessage::User(text) => {
                messages.push(json!({
                    "role": "user", 
                    "content": text
                }));
            },
            ChatMessage::Assistant(text) => {
                messages.push(json!({
                    "role": "assistant", 
                    "content": text
                }));
            },

            _ => {} //need to add tool use. 
        }
    }


    let body = json!({
        "model": config.model,
        "max_tokens": 1600, // you can change if you are rich. Maybe we can add this to config
        "system": config.system_prompt,// specilized for anthropic. Like signiture.
        "messages": messages
    });

    let response: Value = reqwest::Client::new()
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    println!("{response:#}");

    let mut answer = String::new();
    
    if let Some(blocks) = response["content"].as_array() {
        for block in blocks {
            if block["type"] == "text" {
                answer.push_str(block["text"].as_str().unwrap_or(""));
            }
        }
    }

    Ok(Reply::Text(answer))

}
