use super::ToolSpec;
use serde_json::{json, Value};

pub(super) fn spec() -> ToolSpec {

    ToolSpec {
        name:"write_file",
        description:"Creates a file, or owerwrites it competly if it exists. Send the full content.",
        parameters: json!({
            "type":"object",
            "properties": {
                "path": {
                    "type": "string", 
                    "description": "Path of the file to re-write or the path we want to create a file."
                },
                "content": {
                    "type": "string",
                    "description":"The full text write into the file."
                }
            },
            "required": ["path","content"]
        }),
    }
}

pub(super) async fn run(args: &Value) -> String{
    let Some(path) = args["path"].as_str() else {
        //return is a must at here. to finish the fn
        return String::from("Error: missing 'path' argument.");
    };

    let Some(content) = args["content"].as_str() else {
        //Same return must problem at here.
        return String::from("Error: missing 'content' argument.");
    };

    // Take the folder part of the path ("notes/today.md" -> "notes")
    // and crate it (and any missing folder above it, so the write
    // bellow does not fail. None only for paths like "/" with no parent.
    if let Some(parent) = std::path::Path::new(path).parent(){
        if let Err(e) = tokio::fs::create_dir_all(parent).await {
            return format!("Error: could not create folder for '{path}': {e}");
        }
    };

    match tokio::fs::write(path, content).await {
        Ok(()) => format!("Wrote {} bytes to '{path}'", content.len()),
        Err(e) => format!("Error: could not write '{path}' : {e}"),
    }

}
