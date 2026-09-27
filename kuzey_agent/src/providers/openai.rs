use super::{ChatMessage, ProviderConfig, Reply};
use crate::tools::{ToolCall, ToolSpec};
use serde_json::{Value, json};

pub(super) async fn send(
    config: &ProviderConfig,
    history: &[ChatMessage],
    tools: &[ToolSpec],
) -> Result<Reply, Box<dyn std::error::Error>> {
    let api_key = config
        .api_key
        .as_deref()
        .ok_or("OpenAI needs an api_key!")?;

    let mut messages: Vec<Value> = vec![json!({
        "role": "system",
        "content": config.system_prompt
    })];

    for entry in history {
        match entry {
            ChatMessage::User(text) => messages.push(json!({
                "role": "user",
                "content": text,
            })),
            ChatMessage::Assistant(text) => {
                messages.push(json!({
                    "role": "assistant",
                    "content": text,
                }));
            }
            _ => {}
        }
    }

    let tool_list: Vec<Value> = tools
        .iter()
        .map(|t| {
            json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters,
                }
            })
        })
        .collect();

    //Add tool_list when the messages have the integartion of tools.
    let body = json!({
        "model": config.model,
        "messages": messages,
        "stream": false,
    });

    let client = reqwest::Client::new();

    let response: Value = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    println!("{response}");

    Ok(Reply::Text(
        response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string(),
    ))
}
