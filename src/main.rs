// #![allow(non_snake_case)] 等其他模块写好之后，再填上这个。main函数不写逻辑，写到其他模块
use web3::{
    futures::TryStreamExt, 
    transports::WebSocket, 
    types::TransactionId
};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> web3::Result {
    dotenv().ok();
    let wss_url = env::var("WSS").expect("Init the .env file first");

    println!("init websocket");
    let wss_node_endpoint = wss_url.to_string();
    let sub_transport = WebSocket::new(wss_node_endpoint.as_str()).await?;
    let web3 = web3::Web3::new(sub_transport);
    println!("init websocket successfully");
    
    println!("start listenning mempool");
    let mut pending_transactions = web3.eth_subscribe().subscribe_new_pending_transactions().await?;
    
    while let Some(pending_transaction_hash) = pending_transactions.try_next().await? {
        let pth = TransactionId::from(pending_transaction_hash);

        let res = web3.eth().transaction(pth).await;
        match res {
            Ok(opt_txn) => {
                match opt_txn {
                    None => println!("could not find transaction for now\n"),
                    Some(txn) => println!("{:?}\n", txn)
                }
            }
            Err(e) => println!("{:?}\n", e)
        }
    }

    Ok(())
}

