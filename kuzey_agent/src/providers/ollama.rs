use super::{ProviderConfig, ChatMessage, Reply};
use crate::tools::ToolSpec;
use serde_json::{json, Value};

pub(super) async fn send(_config: &ProviderConfig, _history: &[ChatMessage], _tools: &[ToolSpec]) -> Result<Reply, Box<dyn std::error::Error>> {

    //This needs to change. it only takes the latest user message.
    let Some(ChatMessage::User(text)) = _history.last() else {
        return Err("last message is not from the user".into());
    };

    let body = json!({
        "model": _config.model,
        "messages": [ { 
            "role": "user", 
            "content": text
        } ],
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

    // The end part is pretty-print, to see the shape. but is it build in or not I do not know. 
    println!("{response:#}");

    let answer = response["message"]["content"].as_str().unwrap_or("").to_string();

    Ok(Reply::Text(answer))


}
