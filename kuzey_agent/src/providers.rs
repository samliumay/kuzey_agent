mod google;
mod ollama;
mod anthropic;
mod openai;

//Current Google is under construction.
pub enum ProviderKind {
    Ollama,
    Anthropic,
    Google, 
    OpenAI
}

pub struct  AvailableProviderAndModels{

    pub provider_list: Vec<String>,
    pub ollama_models: Vec<String>,
    pub anthropic_models: Vec<String>,
    pub google_models: Vec<String>,
    pub openai_models: Vec<String>,
}

impl  AvailableProviderAndModels{

    pub fn new() -> Self{
    
        Self {
            provider_list: vec!["ollama".to_string(),"openai".to_string(),"anthropic".to_string(),"google".to_string()],
            ollama_models: vec!["qwen3.8".to_string()],
            anthropic_models: vec!["opus-5.0".to_string()],
            google_models: vec!["models/gemini-3.8-flash".to_string()],
            openai_models: vec!["gpt-5.6-astra".to_string()],
        }
    }
}

// We can change the f32 part. needs a performace resarch. 
pub struct ProviderConfig {

    pub kind: ProviderKind,
    pub model: String,
    pub temp: f32,
    pub top_p: f32,
    pub top_k: u32,
    pub api_key: Option<String>,
}

impl ProviderConfig {
    
    pub fn new(kind: ProviderKind, model: String, api_key: Option<String>) -> Self {
        Self {
            kind,
            model,
            temp: 0.7,
            top_p: 0.95,
            top_k: 40,
            api_key,
        }

    }
    
}

pub enum Role {
    User,
    Assistant,
}

// Content type can change.
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

pub async fn send(config: &ProviderConfig, history: &[ChatMessage]) -> Result<String, Box<dyn std::error::Error>>{
    match config.kind {
        ProviderKind::Google => google::send(config, history).await,
        ProviderKind::Ollama => ollama::send(config, history).await,
        ProviderKind::OpenAI => openai::send(config, history).await,
        ProviderKind::Anthropic => anthropic::send(config, history).await,
        _ => Err("provider not implemented yet.".into()),
    }
}





