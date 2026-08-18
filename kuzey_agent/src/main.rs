mod clients;

use std::io;
use clients::ollama_client::example_function;
fn main() {

    println!("Kuzey Agent 0.1.0");

    loop {
        println!("you >");

        let mut your_input = String::new();

        io::stdin().read_line(&mut your_input).expect("Failed to read the line!");


        let user_input = your_input.trim();

        println!("Currently the agent is under construction. But we can print what you wrote");
        println!("{user_input}");

        match user_input {
            "/exit" => {
                println!("exiting...");
                break;
            },

            _ => {
                println!("Unknown command. Your command: {user_input}")
            }
        }

        example_function()
    }
}
