use super::ToolSpec;
use serde_json::{json, Value};

pub(super) fn spec() -> ToolSpec {
    ToolSpec {
        name: "get_current_time",
        description: "Returns the current local date and time.",
        parameters: json!({ "type": "object", "properties": {} }),
    }
}

pub(super) async fn run(_args: &Value) -> String {
    //Looks like returns os time. chekc it out. 
    chrono::Local::now().to_rfc3339()
}
