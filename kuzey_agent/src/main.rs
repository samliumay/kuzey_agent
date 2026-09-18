mod clients;

use std::io;

struct provider {
    model: String,
    temp: f64,
    top_p: u64,
    top_k: u64,
    API_key: String,
}

enum PossibleProviders {
    ollama(provider),
    anthropic(provider),
    google(provider),
    openai(provider),
}

fn detecting_the_provider(provider_name: &str){
}

fn main() {

    println!("Kuzey Agent 0.1.0");

    loop {

        println!("Please enter what provider you will use: ");

        let mut your_input = String::new();

        io::stdin()
            .read_line(&mut your_input)
            .expect("Failed to read the line!");

        //Match with detecting the provider should return spesific enum at provider. with the
        //required config with the controls of unvalid variable. 

        //Needs to be implemented
        println!("Please enter the configs you want.");


        let user_input = your_input.trim();

        println!("Currently the agent is under construction. But we can print what you wrote");
        println!("{user_input}");

        match user_input {
            "/exit" => {
                println!("exiting...");
                break;
            },

            _ => {
                println!("Unknown command. Your command: {user_input}");
            }
        }

        
    }
}
