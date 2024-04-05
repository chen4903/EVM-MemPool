use crate::utils::tools;
use crate::listener::fetcher;

use ethers::{
    core::types::BlockNumber,
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

pub async fn message_rebot(
    api_key: String,
    WSS: String,
    address: String,
    sender: String,
    receiver: String,
    title: String,
    content: String,
    password: String,
    smtp_server: String
) -> Result<()> {
    let client =
    Provider::<Ws>::connect(WSS)
            .await?;
    let client = Arc::new(client);

    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();

    let mut stream = client.subscribe_blocks().await?;

    while let Some(log) = stream.next().await {
        println!("block height: {:?}", log.number);
        let height= *(log.number.unwrap().0.get(0).unwrap());
        let txs = fetcher::fetch_address_all_txs(api_key.clone(), address.as_str(), height, height).await;

        if txs.unwrap().len() > 0 {
            tools::send_email(sender.clone(), receiver.clone(), title.clone(), content.clone(), password.clone(), smtp_server.clone()).unwrap();
        }
    }

    Ok(())
}