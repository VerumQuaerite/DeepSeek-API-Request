use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct DeepSeekRequest {
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let api_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY not set in .env");
    let url = "https://api.deepseek.com/v1/completions";

    let request = DeepSeekRequest {
        prompt: "Explain Rust in simple terms.".to_string(),
        max_tokens: 100,
        temperature: 0.7,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let api_response: DeepSeekResponse = response.json().await?;
        for choice in api_response.choices {
            println!("Response: {}", choice.text);
        }
    } else {
        println!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}
