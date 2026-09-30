mod read_file;
mod run_command;
mod time;
mod write_file;

use inquire::{Select, error::InquireError};

pub struct ToolSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub parameters: serde_json::Value,
}

pub struct ToolCall {
    pub id: Option<String>, //OpenAI and Anthropic give ID to tool calls.
    pub name: String,
    pub args: serde_json::Value,
    // Opaque data Gemini attaches to a call; must be sent back unchanged.
    // This is kinda wierd. needs a resarch.
    pub signature: Option<String>,
}

// returns all the tools we have.
pub fn specs() -> Vec<ToolSpec> {
    vec![
        time::spec(),
        read_file::spec(),
        write_file::spec(),
        run_command::spec(),
    ]
}

fn needs_aproval(name: &str) -> bool {
    matches!(name, "write_file" | "run_command")
}

pub async fn execute(call: &ToolCall) -> String {
    //Not sure &call.name or call.name.
    if needs_aproval(&call.name) {
        let options_for_tool_call: Vec<&str> = vec!["yes", "no"];

        let question_of_tool_call = format!("Allow ? \n{} \n{}", &call.name, &call.args);
        let given_choice_to_do_tool_call: Result<&str, InquireError> =
            Select::new(&question_of_tool_call, options_for_tool_call).prompt();

        let answer_of_choice: bool;

        match given_choice_to_do_tool_call {
            Ok(given_choice_to_do_tool_call) => {
                if given_choice_to_do_tool_call == "yes" {
                    answer_of_choice = true;
                } else {
                    answer_of_choice = false;
                }
            }
            // Ctrl+C / Esc or a terminal problem: treat it as "no".
            Err(_) => {
                return String::from("Tool Call declined: the approval prompt was cancelled.");
            }
        }
        if !answer_of_choice {
            return String::from("Tool Call declined for this tool call.");
        }
    }

    match call.name.as_str() {
        "get_current_time" => time::run(&call.args).await,
        "read_file" => read_file::run(&call.args).await,
        "write_file" => write_file::run(&call.args).await,
        "run_command" => run_command::run(&call.args).await,
        other => format!("Error: unknown tool '{other}'"),
    }
}
