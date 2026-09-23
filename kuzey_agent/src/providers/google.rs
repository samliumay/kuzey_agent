//Getting enums from the providers.rs
use super::{ProviderConfig, ChatMessage, Reply};
// same but getting from tools.
use crate::tools::{ToolCall, ToolSpec};
//Getting gemini related enums ext. You can check the documentation. But its not too easy to
//understand. 
use gemini_rust::{
    Content, FunctionCall, FunctionCallingMode, FunctionDeclaration, Gemini, Message, Part, Role,
    Tool,
};
//json related operations at rust. 
use serde_json::json;

//What this allows does? Also we need to re-write this when we have time with documentation.
//Functiun signitures are interesting. they are vec at original but at here they were list
//references. But you can use as list at here and again convert to vec for capabilities to grow. 
#[allow(deprecated)]
pub(super) async fn send(config: &ProviderConfig, history: &[ChatMessage], tools: &[ToolSpec]) -> Result<Reply, Box<dyn std::error::Error>>{

    //if api ket exists ok or it returns error and inside it says google needs an api key.
    //questions mark indicatres that but still can be re-factored with {}.
    let api_key = config.api_key.as_deref().ok_or("Google needs an API key!")?;

    //Establishing the client. 
    let client = Gemini::with_model(api_key, config.model.clone())?;

    // Translate our history into Gemini's Message type. This is what documentation indicates. they
    // have their own enums ext. 
    let mut messages: Vec<Message> = Vec::new();

    //checks all history.
    for entry in history {

        //if it match with somethig it adds based on type of the information.
        //User,Assistant,toolcall anything you think. too bulky. Maybe possible to refactor. 
        match entry {
            ChatMessage::User(text) => messages.push(Message::user(text)),
            ChatMessage::Assistant(text) => messages.push(Message::model(text)),

            // The model asked for tools: one "model" message holding one part per call.
            ChatMessage::ToolCalls(calls) => {
                //c is the opbject iterated. than its mapped with function calls we want. name is
                //barrowd and args is cloned. not sure why I did that. maybe refactorable. We can
                //also clone the name. 
                let parts = calls.iter().map(|c| Part::FunctionCall {
                    function_call: FunctionCall::new(&c.name, c.args.clone()),
                    thought_signature: c.signature.clone(),
                }).collect();

                //Then its pushes the 'parts' basically function call itself. and the model role
                //which is probably assistant or just enum of model. Again content is from rust
                //doc. a wierd design but its what it is. 
                messages.push(Message {
                    content: Content { parts: Some(parts), role: Some(Role::Model) },
                    role: Role::Model,
                });
            }

            // A tool result. Gemini wants all results of one round in a single "user"
            // message, so append to the previous one if it is also a result message.
            // This is still a wierd syntax I need to study and understand. 
            ChatMessage::ToolResult { name, output } => {
                let part = Part::FunctionResponse {
                    function_response: gemini_rust::FunctionResponse::new(
                        name,
                        json!({ "output": output }),
                    ),
                };
                match messages.last_mut() {
                    Some(Message { content: Content { parts: Some(parts), .. }, .. })
                        if matches!(parts.last(), Some(Part::FunctionResponse { .. })) => {
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
        .with_system_instruction(config.system_prompt.clone())
        .with_temperature(config.temp)
        .with_top_p(config.top_p)
        .with_top_k(config.top_k as i32);

    //Defines our tools in gemini side format. maybe can be a different function. to avoid big
    //functions. 
    for spec in tools {
        let declaration = FunctionDeclaration::new(spec.name, spec.description, None)
            .with_parameters_value(spec.parameters.clone());
        request = request.with_tool(Tool::new(declaration));
    }

    //same we can add this 
    if !tools.is_empty() {
        request = request.with_function_calling_mode(FunctionCallingMode::Auto);
    }

    //it executes the request to google side at here. 
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
    
    //check is it requested tool calls. if its not it retunrns basically. thats all. 
    if calls.is_empty() {
        Ok(Reply::Text(response.text()))
    } else {
        Ok(Reply::ToolCalls(calls))
    }
}
