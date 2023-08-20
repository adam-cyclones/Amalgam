use dotenv::dotenv;
use reqwest;
use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let open_ai_key = env::var("OPENAI_API_KEY").expect("You've not set the OPENAI_API_KEY");

    let client = reqwest::Client::new();

    // Set the API endpoint
    let url = "https://api.openai.com/v1/chat/completions";

    // Define the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", open_ai_key).parse().unwrap(),
    );

    let args: Vec<String> = env::args().collect();

    let my_arg = match args.get(1) {
        Some(arg) => arg,
        None => {
            eprintln!("Error: No `target_file` argument provided."); // Print to stderr
            process::exit(1); // Non-zero exit code indicates an error
        }
    };

    let target_file = fs::read_to_string(my_arg)?;

    // Set up the HTTP POST data
    let post_data = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": format!("{}{}", "Gherkin for: ", target_file)}],
        "temperature": 0.7
    });

    // Make the request
    let response = client
        .post(url)
        .headers(headers)
        .json(&post_data)
        .send()
        .await?
        .text()
        .await?;

    // Parse the JSON body
    let json: Value = serde_json::from_str(&response).unwrap();

    // Get the values of specific fields
    if let Some(choices) = json["choices"].as_array() {
        if let Some(first_choice) = choices.first() {
            if let Some(gherkin) = first_choice["message"]["content"].as_str() {
                println!("{}", gherkin);
            } else {
                println!("content field is not a string or doesn't exist");
            }
        } else {
            println!("choices array is empty");
        }
    } else {
        println!("choices does not exist or is not an array");
    }

    Ok(())
}
