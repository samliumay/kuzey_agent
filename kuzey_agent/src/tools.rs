mod time;
mod read_file;


pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

pub struct ToolCall {
    pub name: String,
    pub args:serde_json::Value,
}

// returns all the tools we have.
pub fn specs() -> Vec<ToolCall> {

}

pub async fn execute(call: &ToolCall) -> String {

    match call.name.as_str(){
        "get_current_time"  => time::run(&call.args).await,
        "read_file"         => read_file::run(&call.args).await,
        other               => format!("Error: unknown tool{other}"),

    }

}
