#![allow(non_snake_case)] 
mod listener;
mod execute;
mod tools;

#[tokio::main]
async fn main() -> web3::Result {
    let _ = listener::listen::listen_mem().await;
    Ok(())
}