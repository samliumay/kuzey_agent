use super::{ChatMessage, ProviderConfig, Reply};
use crate::tools::{ToolCall, ToolSpec};
use serde_json::{Value, json};

pub(super) fn all_available_models() -> Vec<&'static str> {
    vec!["deepseek-flash"]
}

pub(super) async fn send(
    config: &ProviderConfig,
    history: &[ChatMessage],
    tools: &[ToolSpec],
) -> Result<Reply, Box<dyn std::error::Error>> {
    let api_key = config
        .api_key
        .as_deref()
        .ok_or("Deepseek needs an API key!")?;

    let mut input: Vec<Value> = Vec::new();

    // from documentation this seems litle bit different from others.
    for entry in history {
        match entry {
            ChatMessage::User(text) => input.push(json!({
                "role": "user",
                "content": text,
            })),
            ChatMessage::Assistant(text) => input.push(json!({
                "role": "assistant",
                "content": text,
            })),
            ChatMessage::ToolCalls(calls) => {
                for c in calls {
                    input.push(json!({
                        "type": "function_call",
                        "name": c.name,
                        "arguments": c.args.to_string(),
                    }));
                }
            }

            ChatMessage::ToolResult { id, name, output } => {
                input.push(json!({
                    "type": "function_call_output",
                    "call_id": id,
                    "output": output
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
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            })
        })
        .collect();

    //not sure do we need the store false. not sure. Also not sure how openai compatible the
    //deepseek endpoint are. so maybe the back for loop could need a check, not sure.
    let body = json!({
        "model": config.model,
        "instructions": config.system_prompt,
        "input": input,
        "tools": tool_list,
    });

    let client = reqwest::Client::new();

    let response: Value = client
        .post("https://api.deepseek.com")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    Ok(Reply::Text("Under development!".to_string()))
}
