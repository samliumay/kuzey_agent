use super::{ProviderConfig, ChatMessage, Reply};
use crate::tools::{ToolCall, ToolSpec};
use gemini_rust::{
    Content, FunctionCall, FunctionCallingMode, FunctionDeclaration, Gemini, Message, Part, Role,
    Tool,
};
use serde_json::json;

//What this allows does? Also we need to re-write this when we have time with documentation. 
#[allow(deprecated)]
pub(super) async fn send(config: &ProviderConfig, history: &[ChatMessage], tools: &[ToolSpec]) -> Result<Reply, Box<dyn std::error::Error>>{

    let api_key = config.api_key.as_deref().ok_or("Google needs an API key!")?;
    let client = Gemini::with_model(api_key, config.model.clone())?;

    // Translate our history into Gemini's Message type.
    let mut messages: Vec<Message> = Vec::new();
    for entry in history {
        match entry {
            ChatMessage::User(text) => messages.push(Message::user(text)),
            ChatMessage::Assistant(text) => messages.push(Message::model(text)),

            // The model asked for tools: one "model" message holding one part per call.
            ChatMessage::ToolCalls(calls) => {
                let parts = calls.iter().map(|c| Part::FunctionCall {
                    function_call: FunctionCall::new(&c.name, c.args.clone()),
                    thought_signature: c.signature.clone(),
                }).collect();
                messages.push(Message {
                    content: Content { parts: Some(parts), role: Some(Role::Model) },
                    role: Role::Model,
                });
            }

            // A tool result. Gemini wants all results of one round in a single "user"
            // message, so append to the previous one if it is also a result message.
            ChatMessage::ToolResult { name, output } => {
                let part = Part::FunctionResponse {
                    function_response: gemini_rust::FunctionResponse::new(
                        name,
                        json!({ "output": output }),
                    ),
                };
                match messages.last_mut() {
                    Some(Message { content: Content { parts: Some(parts), .. }, .. })
                        if matches!(parts.last(), Some(Part::FunctionResponse { .. })) =>
                    {
                        parts.push(part);
                    }
                    _ => messages.push(Message {
                        content: Content { parts: Some(vec![part]), role: Some(Role::User) },
                        role: Role::User,
                    }),
                }
            }
        }
    }

    // Translate our tool specs into Gemini's function declarations.
    let mut request = client
        .generate_content()
        .with_messages(messages)
        .with_temperature(config.temp)
        .with_top_p(config.top_p)
        .with_top_k(config.top_k as i32);

    for spec in tools {
        let declaration = FunctionDeclaration::new(spec.name, spec.description, None)
            .with_parameters_value(spec.parameters.clone());
        request = request.with_tool(Tool::new(declaration));
    }
    if !tools.is_empty() {
        request = request.with_function_calling_mode(FunctionCallingMode::Auto);
    }

    let response = request.execute().await?;

    // Did the model ask for tools, or answer with text?
    let calls: Vec<ToolCall> = response
        .function_calls_with_thoughts()
        .into_iter()
        .map(|(call, signature)| ToolCall {
            name: call.name.clone(),
            args: call.args.clone(),
            signature: signature.cloned(),
        })
        .collect();

    if calls.is_empty() {
        Ok(Reply::Text(response.text()))
    } else {
        Ok(Reply::ToolCalls(calls))
    }
}
