mod providers;
 
use std::io;
use inquire::{Select, MultiSelect, Password, PasswordDisplayMode};
use providers::ProviderKind;
use providers::ProviderConfig;
use providers::send;


#[tokio::main]
async fn main() {

    //Problmematic desing performance and architecture wise. Needs to chjange.
    let all_providers: Vec<&str> = vec!["ollama","google","openai","anthropic"];

    println!("Kuzey Agent 0.1.0");
    
    let provider_type= Select::new("Which provider are you going with!", all_providers)
        .prompt()
        .unwrap();

    println!("Selected {provider_type}");
    
    let kind = match provider_type {
        "ollama" => ProviderKind::Ollama,
        "google" => ProviderKind::Google,
        "openai" => ProviderKind::OpenAI,
        "anthropic" => ProviderKind::Anthropic,
        _ => unreachable!("Select only returns item from the list")
    };

    let mut api_key = Password::new("Please enter your API key:")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();

    // Same problem for here. maybe with a constructor or something like that but this is really
    // problematic design to go with. Its basically wrong and will create trouble at more
    // integrations. 
    let all_google_models: Vec<&str> = vec!["models/gemini-3.8-flash"];

    let model_type = Select::new("Which model you want to use!", all_google_models)
        .prompt()
        .unwrap();

    let model_type = model_type.to_string();

    let providerDetails = ProviderConfig::new(kind, model_type, Some(api_key));

    println!("Enter your first prompt!");
    let mut prompt = String::new();

    io::stdin()
        .read_line(&mut prompt)
        .expect("failed to read the line.");
    
    let results:Result<String, Box<dyn std::error::Error>> = providers::send(&providerDetails, &prompt).await;
    
    match results{
        Ok(text) => println!("{text}"),
        Err(e) => println!("Error {e}"),
    };
    

}
