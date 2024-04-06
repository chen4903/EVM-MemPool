use ethers::{
    core::{
        abi::AbiDecode,
        types::{Address, BlockNumber, Filter, U256},
    },
    providers::{Middleware, Provider, StreamExt, Ws},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use eyre::Result;
use crate::listener::fetcher::fetch_address_all_txs;
use crate::utils::tools;

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

struct Listen {
    pub WSS: String,
    pub API_Key: String,
}

impl Listen {

    pub fn new(wss: String, api_key: String) -> Self {
        Listen {
            WSS: wss,
            API_Key: api_key,
        }
    }

    // 监控某个地址是否有ERC20转账
    pub async fn subscribe_erc20_transfer(&self, address: String) -> Result<()> {
        let client =
        Provider::<Ws>::connect(self.WSS.clone()).await?;
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
    
    pub async fn subscribe_address(&self, address: String) -> Result<()> {
        let client = Provider::<Ws>::connect(self.WSS.clone()).await?;
        let client = Arc::new(client);
    
        let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    
        let mut stream = client.subscribe_blocks().await?;
    
        while let Some(log) = stream.next().await {
            println!("block height: {:?}", log.number);
            let height= *(log.number.unwrap().0.get(0).unwrap());
            let _ = fetch_address_all_txs(self.API_Key.clone(), address.as_str(), height, height).await;
        }
    
        Ok(())
    }
    
    pub async fn monitor_mixing_service(&self) -> Result<()> {
        // 获取mixing_service数据
        let mixing_services = tools::get_db_address("mixing_service");
    
        let client = Provider::<Ws>::connect(self.WSS.clone()).await?;
        let client = Arc::new(client);
    
        let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    
        let mut stream = client.subscribe_blocks().await?;
    
        while let Some(log) = stream.next().await {
            println!("block height: {:?}", log.number);
            let height= *(log.number.unwrap().0.get(0).unwrap());
            for address in &mixing_services { // 遍历所有的混币器地址
                let txs = fetch_address_all_txs(self.API_Key.clone(), (*address).as_str(), height, height).await;
    
                for tx in txs.unwrap() { // 如果在最新的区块当中，有混币器的交易，则打印出来
                    // 把存钱进混币器的用户，记录下来
                    tools::write_addresses_db(tx.from);
                }
            }
        }
    
        Ok(())
    }
    
}