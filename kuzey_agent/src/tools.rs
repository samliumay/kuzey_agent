mod time;
mod read_file;


pub struct ToolSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub parameters: serde_json::Value,
}

pub struct ToolCall {
    pub name: String,
    pub args: serde_json::Value,
    // Opaque data Gemini attaches to a call; must be sent back unchanged.
    // This is kinda wierd. needs a resarch. 
    pub signature: Option<String>,
}

// returns all the tools we have.
pub fn specs() -> Vec<ToolSpec> {
    vec![time::spec(), read_file::spec()]
}

pub async fn execute(call: &ToolCall) -> String {

    match call.name.as_str(){
        "get_current_time"  => time::run(&call.args).await,
        "read_file"         => read_file::run(&call.args).await,
        other               => format!("Error: unknown tool '{other}'"),

    }

}
