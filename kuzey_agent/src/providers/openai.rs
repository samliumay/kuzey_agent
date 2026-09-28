use super::{ChatMessage, ProviderConfig, Reply};
use crate::tools::{ToolCall, ToolSpec};
use serde_json::{Value, json};

// Uses the Responses API (/v1/responses). Chat Completions rejects function tools
// for reasoning models unless reasoning_effort is "none".
pub(super) async fn send(
    config: &ProviderConfig,
    history: &[ChatMessage],
    tools: &[ToolSpec],
) -> Result<Reply, Box<dyn std::error::Error>> {
    let api_key = config
        .api_key
        .as_deref()
        .ok_or("OpenAI needs an api_key!")?;

    // "input" is a flat list of items. The system prompt is not an item; it goes
    // to "instructions" below.
    let mut input: Vec<Value> = Vec::new();

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
            // One "function_call" item per call, not one message holding all calls.
            // Only call_id is sent back: sending the item's own "fc_..." id would make
            // OpenAI expect the matching reasoning item too.
            ChatMessage::ToolCalls(calls) => {
                for c in calls {
                    input.push(json!({
                        "type": "function_call",
                        "call_id": c.id,
                        "name": c.name,
                        "arguments": c.args.to_string(), // must be a JSON string
                    }));
                }
            }
            ChatMessage::ToolResult { id, output, .. } => input.push(json!({
                "type": "function_call_output",
                "call_id": id,
                "output": output,
            })),
        }
    }

    // Same as Chat Completions, but flat: no nested "function" object.
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

    let body = json!({
        "model": config.model,
        "instructions": config.system_prompt,
        "input": input,
        "tools": tool_list,
        "store": false, // we send the whole history every time, so no need to keep it on OpenAI's side
    });

    let client = reqwest::Client::new();

    //Check the endooint validity from https://developers.openai.com/api/docs?lang=curl
    let response: Value = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    // println!("{response:#}");

    // Errors come back as JSON too: {"error": {"message": ...}}.
    if let Some(msg) = response["error"]["message"].as_str() {
        return Err(format!("OpenAI error: {msg}").into());
    }

    // "output" is a list of items: "reasoning" (skipped), "function_call" or "message".
    let mut answer = String::new();
    let mut calls: Vec<ToolCall> = Vec::new();

    if let Some(items) = response["output"].as_array() {
        for item in items {
            if item["type"] == "function_call" {
                calls.push(ToolCall {
                    // call_id links the call to its result; "id" is the item's own id.
                    id: item["call_id"].as_str().map(|s| s.to_string()),
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    args: serde_json::from_str(item["arguments"].as_str().unwrap_or("{}"))
                        .unwrap_or(json!({})),
                    signature: None,
                });
            } else if item["type"] == "message"
                && let Some(parts) = item["content"].as_array()
            {
                for part in parts {
                    if part["type"] == "output_text" {
                        answer.push_str(part["text"].as_str().unwrap_or(""));
                    }
                }
            }
        }
    }

    if calls.is_empty() {
        Ok(Reply::Text(answer))
    } else {
        Ok(Reply::ToolCalls(calls))
    }
}
