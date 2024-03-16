#![allow(non_snake_case)] 
use text_to_ascii_art::convert;

mod listener;
mod execute;
mod tools;

#[tokio::main]
async fn main() -> web3::Result {
    match convert("EVM MemPool".to_string()) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    let _ = listener::listen::listen_analysis_all().await;
    Ok(())
}