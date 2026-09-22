use super::ToolSpec;
use serde_json::{json, Value};

pub(super) fn spec() -> ToolSpec {
    ToolSpec {
        name: "read_file",
        description: "Reads a text file from disk and returns its contents.",
        parameters: json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path of the file to read." }
            },
            "required": ["path"]
        }),
    }
}

pub(super) async fn run(args: &Value) -> String {
    // args is a JSON object like {"path": "Cargo.toml"}; pull the string out.
    let Some(path) = args["path"].as_str() else {
        return String::from("Error: missing 'path' argument");
    };

    match tokio::fs::read_to_string(path).await {
        Ok(contents) => contents,
        Err(e) => format!("Error: could not read '{path}': {e}"),
    }
}
