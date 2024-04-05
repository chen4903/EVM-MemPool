use ethers::{
    core::{
        abi::AbiDecode,
        types::{Address, BlockNumber, Filter, U256},
    },
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

use crate::listener::fetcher::fetch_address_all_txs;

pub async fn subscribe_erc20_transfer(
    WSS: String,
    address: String
) -> Result<()> {
    let client =
    Provider::<Ws>::connect(WSS)
            .await?;
    let client = Arc::new(client);

    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    println!("last_block: {last_block}");

    let erc20_transfer_filter =
        Filter::new()
            .to_block(last_block + 100000000) // 监听到哪个区块
            .event("Transfer(address,address,uint256)") // 发出啥事件
            .address(address.parse::<Address>().unwrap()); // 某个地址发出的

    let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?;

    while let Some(log) = stream.next().await {
        // 这里可以做判断，如果是某个用户发送的交易，再返回
        println!(
            "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            Address::from(log.topics[1]),
            Address::from(log.topics[2]),
            U256::decode(log.data)
        );
    }

    Ok(())
}

pub async fn subscribe_address(
    api_key: String,
    WSS: String,
    address: String
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
        let _ = fetch_address_all_txs(api_key.clone(), address.as_str(), height, height).await;
    }

    Ok(())
}