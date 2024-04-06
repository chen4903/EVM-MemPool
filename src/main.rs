#![allow(non_snake_case)]
#![allow(unused_variables)]
mod execute;
mod listener;
mod utils;
mod ai;

use text_to_ascii_art::convert;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match convert("SecHelper".to_string()) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    let wss_url = env::var("WSS_RPC").expect("Init the .env file first");
    let api_key = env::var("ETHERSCAN_API_KEY").expect("Init the .env file first");
    let openai_base_url = env::var("OPENAI_BASE_URL").expect("Init the .env file first");
    let sender = env::var("SENDER").expect("Init the .env file first");
    let openai_key = env::var("OPENAI_KEY").expect("Init the .env file first");
    let receiver = env::var("RECEIVER").expect("Init the .env file first");
    let password = env::var("PASSWORD").expect("Init the .env file first");
    let smtp_sever = env::var("SMTP_SEVER").expect("Init the .env file first");

    Ok(())
}