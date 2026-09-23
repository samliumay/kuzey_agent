use super::{ProviderConfig, ChatMessage, Reply};
use crate::tools::{ToolSpec, ToolCall};
use serde_json::{json, Value};

pub(super) async fn send(_config: &ProviderConfig, _history: &[ChatMessage], _tools: &[ToolSpec]) -> Result<Reply, Box<dyn std::error::Error>> {


let mut messages: Vec<Value> = vec![ json!({
    "role": "system", 
    "content": _config.system_prompt
}) ];

for entry in _history {
    match entry {
        ChatMessage::User(text) => {
            messages.push(json!({
                "role": "user", 
                "content": text,
            }));
        }
        ChatMessage::Assistant(text) => {
            messages.push(json!({
                "role": "assistant",
                "content": text
            }));
        }
        ChatMessage::ToolCalls(calls) => {
            let tool_calls: Vec<Value> = calls.iter().map(|c| json!({
                "function": {
                    "name": c.name, 
                    "arguments": c.args
                }
            })).collect();

            messages.push(json!({"role": "assistant", "tool_calls": tool_calls}));
        }
        ChatMessage::ToolResult {name, output} => {
            messages.push(json!({
                "role": "tool", 
                "tool_name": name, 
                "content": output, 
            
            }));
        }
    }
}

let tool_list: Vec<Value> = _tools.iter().map(|t| json!({
    "type": "function",
    "function": {
        "name": t.name,
        "description": t.description,
        "parameters": t.parameters
    }
})).collect();


    let body = json!({
        "model": _config.model,
        "messages": messages,
        "tools": tool_list,
        "stream": false
    });

    let client = reqwest::Client::new();

    let response: Value = client
        .post("http://localhost:11434/api/chat")
        .json(&body) // turn body to JSON for the request
        .send()
        .await? //Waits for the server and return on error. 
        .json()
        .await?; //read the reply body as json. 

    if let Some(calls) = response["message"]["tool_calls"].as_array() {
        let calls: Vec<ToolCall> = calls.iter().map(|c| ToolCall {
            name: c["function"]["name"].as_str().unwrap_or("").to_string(),
            args: c["function"]["arguments"].clone(),
            signature: None //This is just for gemini.
        }).collect();
        if !calls.is_empty() {
            return Ok(Reply::ToolCalls(calls));
        };
    };

    // The end part is pretty-print, to see the shape. but is it build in or not I do not know. 
    // println!("{response:#}");

    let answer = response["message"]["content"].as_str().unwrap_or("").to_string();

    Ok(Reply::Text(answer))


}
