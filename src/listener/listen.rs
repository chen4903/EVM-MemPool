use web3::{
    futures::StreamExt, 
    types::{TransactionId,H256,H160}
};
use std::{
    env, future,
};
use dotenv::dotenv;
use std::sync::mpsc;
use std::str::FromStr;

// The producer consumer model, as consumers need to retrieve on chain data again, 
// the longer it runs, the higher the latency of consumers
pub async fn listen_analysis_all() -> web3::Result {
    dotenv().ok();
    let wss_url = env::var("WSS").expect("Init the .env file first");

    let ws = web3::transports::WebSocket::new(wss_url.as_str()).await?;
    let web3 = web3::Web3::new(ws.clone());
    let sub = web3
        .eth_subscribe()
        .subscribe_new_pending_transactions()
        .await?;

    let (sender, receiver): (mpsc::Sender<Result<H256,_>>, mpsc::Receiver<Result<H256,_>>) = mpsc::channel();

    tokio::spawn(async move {
        sub.for_each(|hash| {
            sender.send(hash).unwrap();
            future::ready(())
        }).await;
    });

    while let Ok(hash) = receiver.recv() {
        println!("hash:{:?}", hash);

        let tx = TransactionId::from(hash.unwrap());

        let res = web3.eth().transaction(tx).await;
        match res {
            Ok(opt_txn) => {
                match opt_txn {
                    None => println!("could not find transaction for now\n"),
                    Some(txn) => {
                        // Binance hot wallet for example
                        let target_address = H160::from_str("0x28C6c06298d514Db089934071355E5743bf21d60").unwrap();
                        if txn.from == Some(target_address){
                            println!("details: {:?}\n", txn);
                        }
                    }
                }
            }
            Err(e) => println!("{:?}\n", e)
        }
    }

    Ok(())
}