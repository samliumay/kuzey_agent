use super::{ProviderConfig, ChatMessage, Reply};
use crate::tools::ToolSpec;

pub(super) async fn send(_config: &ProviderConfig, _history: &[ChatMessage], _tools: &[ToolSpec]) -> Result<Reply, Box<dyn std::error::Error>> {

    Ok(Reply::Text(String::from("will be implemented")))

}
