#![allow(non_snake_case)]
#![allow(unused_variables)]
use text_to_ascii_art::convert;
mod execute;
mod listener;
mod utils;
mod ai;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("ETHERSCAN_API_KEY").expect("Init the .env file first");
    // let wss_url = env::var("WSS_RPC").expect("Init the .env file first");

    match convert("SecHelper".to_string()) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    // listener::fetcher::fetch_address_normal_txs(api_key.clone(),"0x0A30ccEda7f03B971175e520c0Be7E6728860b67", 0, 99999999999999).await?;
    // listener::fetcher::fetch_address_internal_txs(api_key.clone(),"0x0A30ccEda7f03B971175e520c0Be7E6728860b67", 0, 99999999999999).await?;
    // listener::fetcher::fetch_address_all_txs(api_key.clone(),"0x0A30ccEda7f03B971175e520c0Be7E6728860b67", 0, 99999999999999).await?;

    // utils::tools::get_contract_solidity_code(api_key, "0xB20bd5D04BE54f870D5C0d3cA85d82b34B836405").await?;
    // let is_invoke = listener::fetcher::is_invoke_mixing_service(api_key.clone(),"0x0A30ccEda7f03B971175e520c0Be7E6728860b67", 0, 99999999999999).await?;
    // println!("是否有混币器相关交易:{}", is_invoke);

    // 测试ChatGPT功能
    ai::chatgpt::openai().await;

    Ok(())
}

