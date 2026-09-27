use super::ToolSpec;
use std::time::Duration;
use tokio::process::Command;
use serde_json::{json, Value};

pub(super) fn spec() -> ToolSpec {
    ToolSpec {
        name: "run_command",
        description: "function gives capability to run commands at cmd.",
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "path that will command be executed."
                },
                "command": {
                    "type": "string",
                    "description": "command will be executed."
                }
            },
            "required": ["path", "command"]
        }),
    }
}

pub(super) async fn run(args: &Value) -> String {
    
    let Some(path) = args["path"].as_str() else {
        return String::from("Error: missing 'path' variable.");
    };

    let Some(command) = args["command"].as_str() else {
       return String::from("Error: missing 'command' variable.");  
    };

    match tokio::fs::metadata(path).await{
        Ok(meta) if meta.is_dir() => {},
        Ok(_) => return format!("Error: '{path}' is not a directory."),
        Err(_) => return format!("Error: '{path}' does not exists.")
    }

    //This will be a problem at the fraature for windows. We can work on that 
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(path)
        .kill_on_drop(true)
        .output();

    let output = match tokio::time::timeout(Duration::from_secs(30),child).await {
        Err(_) => return String::from("Error: command timed out after 30 seconds."),
        Ok(Err(e)) => return format!("Error: could not start command {e}"),
        Ok(Ok(output)) => output,
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    format!(
        "exit code: {}\nstdout:{stdout}\nstderr:{stderr}",
        output.status.code().map_or("killed".to_string(), |c| c.to_string())
    )

}
