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
    let client = Provider::<Ws>::connect(WSS).await?;
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

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;


// 定义结构体来表示JSON数据
#[derive(Debug, Serialize, Deserialize)]
struct AddressData {
    eth: Data,
    bsc: Data
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    hacker: Vec<String>,
    protocol: Vec<String>,
    mixing_service: Vec<String>,
    potential_hacker: Vec<String>
}

pub async fn monitor_mixing_service(
    api_key: String,
    WSS: String,
) -> Result<()> {
    // 读取JSON文件
    let file = File::open("src/utils/addresses.json").expect("Failed to open file");
    let reader = BufReader::new(file);

    // 解析JSON数据
    let data: AddressData = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // 获取mixing_service数据
    let mixing_services = data.eth.mixing_service;

    let client = Provider::<Ws>::connect(WSS).await?;
    let client = Arc::new(client);

    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();

    let mut stream = client.subscribe_blocks().await?;

    while let Some(log) = stream.next().await {
        println!("block height: {:?}", log.number);
        let height= *(log.number.unwrap().0.get(0).unwrap());
        for address in &mixing_services { // 遍历所有的混币器地址
            let txs = fetch_address_all_txs(api_key.clone(), (*address).as_str(), height, height).await;

            for tx in txs.unwrap() { // 如果在最新的区块当中，有混币器的交易，则打印出来
                // 把存钱进混币器的用户，记录下来
                writess(tx.from);
            }
        }
    }

    Ok(())
}

fn writess(address: String) {
    let json_file_path = Path::new("src/utils/addresses.json");
    let file = File::open(json_file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // 解析JSON数据
    let mut data: AddressData = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // 将字符串 "123" 添加到 potential_hacker 字段的数组中
    data.eth.potential_hacker.push(address);

    // 写回JSON文件
    let new_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(json_file_path)
        .expect("Failed to open file for writing");
    let mut writer = BufWriter::new(new_file);

    // 将更新后的数据写回文件
    serde_json::to_writer_pretty(&mut writer, &data).expect("Failed to write JSON to file");
}