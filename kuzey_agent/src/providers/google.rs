use super::{ProviderConfig, ChatMessage, Role};
use gemini_rust::{Gemini, Message};


#[allow(deprecated)]
pub(super) async fn send(config: &ProviderConfig, history: &[ChatMessage]) -> Result<String, Box<dyn std::error::Error>>{

    //What is as_deref and question marks at the documentation.?
    let api_key = config.api_key.as_deref().ok_or("Google needs an API key!")?;
    let client = Gemini::with_model(api_key, config.model.clone())?;
    
    //What map does and what it retunrs at here? A touple or list ?
    let messages = history.iter().map(|m| match m.role {
        Role::User => Message::user(&m.content),
        Role::Assistant => Message::model(&m.content),
    });

    //Why still i32 stays? 
    let response = client
        .generate_content()
        .with_messages(messages)
        .with_temperature(config.temp)
        .with_top_p(config.top_p)
        .with_top_k(config.top_k as i32)
        .execute()
        .await?;

    Ok(response.text())

}
