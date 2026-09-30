mod providers;
mod tools;

use inquire::{
    CustomUserError, Password, PasswordDisplayMode, Select, Text,
    autocompletion::{Autocomplete, Replacement},
};
use providers::{ChatMessage, ProviderConfig, ProviderKind, Reply};
use std::path::Path;

#[tokio::main]
async fn main() {
    println!("Kuzey Agent 0.1.0");

    // Ctrl+C / Esc in any prompt returns Err: exit cleanly instead of panicking.
    let Ok(kind) = Select::new(
        "Which provider are you going with?",
        ProviderKind::ALL.to_vec(),
    )
    .prompt() else {
        println!("Exiting...");
        return;
    };

    let api_key = if kind.needs_api_key() {
        let Ok(key) = Password::new("Please enter your API key:")
            .with_display_mode(PasswordDisplayMode::Masked)
            .without_confirmation()
            .prompt() else {
            println!("Exiting...");
            return;
        };
        let key = key.trim();
        if key.is_empty() {
            None
        } else {
            Some(key.to_string())
        }
    } else {
        None
    };

    let Ok(model) = Select::new("Which model do you want to use?", kind.models()).prompt() else {
        println!("Exiting...");
        return;
    };

    let provider_details = ProviderConfig::new(kind, model.to_string(), api_key);

    let tool_specs = tools::specs();
    let mut history: Vec<ChatMessage> = Vec::new();

    loop {
        let Ok(input) = Text::new("You:").prompt() else {
            println!("Exiting...");
            break;
        };
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "/exit" {
            println!("Exiting...");
            break;
        }

        // Where this turn starts, so a failed turn can be removed completely.
        let turn_start = history.len();
        history.push(ChatMessage::User(input.to_string()));

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
                            id: call.id.clone(),
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
                    // pop() removed only the last item. After a tool round that left a
                    // ToolCalls without its ToolResult, and OpenAI/Anthropic reject such a
                    // history on every later request. Drop the whole turn instead.
                    history.truncate(turn_start);
                    break;
                }
            }
        }
    }
}
