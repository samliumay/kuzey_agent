mod providers;
mod tools;
 
use inquire::{Select, Password, PasswordDisplayMode, Text};
use providers::{ProviderKind, ProviderConfig, ChatMessage, Reply};

#[tokio::main]
async fn main() {

    println!("Kuzey Agent 0.1.0");

    let kind = Select::new("Which provider are you going with?", ProviderKind::ALL.to_vec())
        .prompt()
        .unwrap();

    let api_key = if kind.needs_api_key() {
        let key = Password::new("Please enter your API key:")
            .with_display_mode(PasswordDisplayMode::Masked)
            .without_confirmation()
            .prompt()
            .unwrap();
        let key = key.trim();
        if key.is_empty() { None } else { Some(key.to_string()) }
    } else {
        None
    };

    let model = Select::new("Which model do you want to use?", kind.models())
        .prompt()
        .unwrap();

    let provider_details = ProviderConfig::new(kind, model.to_string(), api_key);

    let tool_specs = tools::specs();
    let mut history: Vec<ChatMessage> = Vec::new();

    loop {
        
        let input = Text::new("You:").prompt().unwrap();
        let input = input.trim();

        if input.is_empty(){
            continue;
        }

        if input == "/exit" {
            println!("Exiting...");
            break;
        }

        history.push(
            ChatMessage::User(input.to_string())
        );

        
        // Inner loop: keep going while the model asks for tools.
        // It ends when the model answers with text (or after 10 rounds).
        for _ in 0..10 {
            match providers::send(&provider_details, &history, &tool_specs).await {
                Ok(Reply::Text(text)) => {
                    println!("{text}");
                    history.push(ChatMessage::Assistant(text));
                    break;
                }
                Ok(Reply::ToolCalls(calls)) => {
                    let mut results = Vec::new();
                    for call in &calls {
                        println!("[tool] {} {}", call.name, call.args);
                        let output = tools::execute(call).await;
                        results.push(ChatMessage::ToolResult {
                            name: call.name.clone(),
                            output,
                        });
                    }
                    history.push(ChatMessage::ToolCalls(calls));
                    //This part is kinda wierd. why not push results? 
                    history.extend(results);
                }
                Err(e) => {
                    println!("Error {e}");
                    history.pop();
                    break;
                }
            }
        }
    }

}
