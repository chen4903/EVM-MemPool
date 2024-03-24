#![allow(non_snake_case)]
use text_to_ascii_art::convert;
mod execute;
mod listener;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match convert("EVM MemPool".to_string()) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }
    println!("|||||||||||||||||||||||||||||");
    println!(" GetAddressTransaction Test");
    println!("|||||||||||||||||||||||||||||");
    let addr = "0xbE4bf446e2Bdd6ebaD529A4df21911c87E48E535";
    listener::get_transaction_by_address::fetch_transactions(addr, 0, 99999999).await?;

    println!("|||||||||||||||||||||||||||||");
    println!(" Memory Pool Listen Test");
    println!("|||||||||||||||||||||||||||||");
    let _ = listener::listen::listen_analysis_all().await;
    Ok(())
}
