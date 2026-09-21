use super::{ProviderConfig, ChatMessage, Role};


#[allow(deprecated)]
pub(super) async fn send(config: &ProviderConfig, history: &[ChatMessage]) -> Result<String, Box<dyn std::error::Error>>{

    Ok(String::from("will be implemented"))

}

