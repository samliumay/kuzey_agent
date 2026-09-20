mod google;

//Current Google is under construction.
pub enum ProviderKind {
    Ollama,
    Anthropic,
    Google, 
    OpenAI
}

pub struct ProviderConfig {

    pub kind: ProviderKind,
    pub model: String,
    pub temp: f64,
    pub top_p: f64,
    pub top_k: u64,
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

pub async fn send(config: &ProviderConfig, prompt: &str) -> Result<String, Box<dyn std::error::Error>>{
    match config.kind {
        ProviderKind::Google => google::send(config, prompt).await,
        _ => Err("provider not implemented yet.".into()),
    }
}

