mod providers;
 
use std::io;
use inquire::{Select, Password, PasswordDisplayMode};
use providers::ProviderKind;
use providers::ProviderConfig;
use providers::send;
use providers::AvailableProviderAndModels;

#[tokio::main]
async fn main() {

    let availables = AvailableProviderAndModels::new();

    println!("Kuzey Agent 0.1.0");
    
    let provider_type= Select::new("Which provider are you going with!",availables.provider_list )
        .prompt()
        .unwrap();

    println!("Selected {provider_type}");
    
    let kind = match provider_type.as_str() {
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

    let model_type = match kind {
        
        ProviderKind::Ollama => {
            let model_type = Select::new("Which model you want to use!", availables.ollama_models)
            .prompt()
            .unwrap();

            model_type
        },
        ProviderKind::Google => {
            let model_type = Select::new("Which model you want to use!", availables.google_models)
            .prompt()
            .unwrap();

            model_type
        },
        ProviderKind::OpenAI => {
            let model_type = Select::new("Which model you want to use!", availables.openai_models)
            .prompt()
            .unwrap();

            model_type
        },
        ProviderKind::Anthropic => {
            let model_type = Select::new("Which model you want to use!", availables.anthropic_models)
            .prompt()
            .unwrap();

            model_type
        },
        _ => unreachable!("Select only returns item from the list")
    
    };

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
