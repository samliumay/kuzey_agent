use super::ProviderConfig;
use gemini_rust::Gemini;

pub(super) async fn send(config: &ProviderConfig, prompt: &str) -> Result<String, Box<dyn std::error::Error>>{

    //What is as_deref and question marks at the documentation.?
    let api_key = config.api_key.as_deref().ok_or("Google needs an API key!")?;
    let client = Gemini::with_model(api_key, config.model.clone())?;

    //Why f32 at the doc? We created it at f64
    let response = client
        .generate_content()
        .with_user_message(prompt)
        .with_temperature(config.temp as f32)
        .with_top_p(config.top_p as f32)
        .with_top_k(config.top_k as i32)
        .execute()
        .await?;

    Ok(response.text())


    

}
