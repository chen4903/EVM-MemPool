use dotenv::dotenv;
use ethers::core::types::H160;
use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use std::{
    env, future,
    sync::{mpsc, Arc},
};

pub async fn listen_analysis_all() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let wss_url = env::var("WSS").expect("Init the .env file first");

    let provider = Arc::new(Provider::<Ws>::connect(wss_url).await.unwrap());

    let (sender, receiver) = mpsc::channel();

    let provider_clone = Arc::clone(&provider);
    tokio::spawn(async move {
        provider_clone
            .subscribe_pending_txs()
            .await
            .unwrap()
            .for_each(|tx_hash| {
                sender.send(tx_hash).unwrap();
                future::ready(())
            })
            .await;
    });

    // producer:comsumer = 1:1
    // TODO: producer:comsumer = 1:n
    while let Ok(tx_hash) = receiver.recv() {
        println!("hash:{:?}", tx_hash);
        if let Ok(receipt) = provider.get_transaction(tx_hash).await {
            let target_address: H160 = "0x28C6c06298d514Db089934071355E5743bf21d60"
                .parse()
                .unwrap(); // e.g. Binance hot wallet
            if receipt.clone().unwrap().from == target_address {
                println!("details: {:?}", receipt);
            }
        } else {
            println!(
                "Failed to fetch transaction receipt for hash: {:?}",
                tx_hash
            );
        }
    }

    Ok(())
}
