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
    let wss_url = env::var("WSS_RPC").expect("Init the .env file first");

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
    // ai::chatgpt::openai().await;

    // 测试监听功能
    // listener::listen::hello().await;

    // 测试发邮件功能
    // let sender = env::var("SENDER").expect("Init the .env file first");
    // let receiver = env::var("RECEIVER").expect("Init the .env file first");
    // let password = env::var("PASSWORD").expect("Init the .env file first");
    // let smtp_server = env::var("SMTP_SEVER").expect("Init the .env file first");
    // utils::tools::send_email(
    //     sender,
    //     receiver,
    //     String::from("SecHelper robot"),
    //     String::from("Your asset may be in dangerous!"),
    //     password,
    //     smtp_server
    // ).unwrap();

    // 监听某个地址，发出的事件
    // listener::listen::subscribe_erc20_transfer(
    //     wss_url.clone(), 
    //     "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string() // USDT
    // ).await?;

    // 监听某个地址的所有交易动向
    // listener::listen::subscribe_address(
    //     api_key,
    //     wss_url.clone(), 
    //     "0x28C6c06298d514Db089934071355E5743bf21d60".to_string() // Binance hot wallet 14
    // ).await?;

    // 监听某个地址, 如果他发出了交易，则发出邮件通知
    // let sender = env::var("SENDER").expect("Init the .env file first");
    // let receiver = env::var("RECEIVER").expect("Init the .env file first");
    // let password = env::var("PASSWORD").expect("Init the .env file first");
    // let smtp_server = env::var("SMTP_SEVER").expect("Init the .env file first");
    // execute::guardian::message_rebot(
    //     api_key,
    //     wss_url.clone(), 
    //     "0x28C6c06298d514Db089934071355E5743bf21d60".to_string(), // Binance hot wallet 14
    //     sender,
    //     receiver,
    //     String::from("SecHelper robot"),
    //     String::from("Binance hot wallet 14 has action!"),
    //     password,
    //     smtp_server
    // ).await?;
    
    Ok(())
}