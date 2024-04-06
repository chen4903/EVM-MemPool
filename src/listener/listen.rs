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
use crate::listener::fetcher;
use crate::utils::tools;

/// @dev：Used to parse the data For addresses.json
#[derive(Debug, Serialize, Deserialize)]
struct AddressData {
    eth: Data,
    bsc: Data
}

/// @dev：Used to parse the data For addresses.json
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

    /// @param wss WSS URL
    /// @param api_key Etherscan API kEY
    pub fn new(wss: String, api_key: String) -> Self {
        Listen {
            WSS: wss,
            API_Key: api_key,
        }
    }

    /// @dev Monitor a certain address if it has ERC20 transfer tx
    /// @param address The address to monitor
    pub async fn subscribe_erc20_transfer(&self, address: String) -> Result<()> {
        let client =
        Provider::<Ws>::connect(self.WSS.clone()).await?;
        let client = Arc::new(client);
    
        let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
        println!("last_block: {last_block}");
    
        let erc20_transfer_filter =
            Filter::new()
                .to_block(last_block + 999999999) // To which block, we just plus 999999999 because we assume that the program will not run continuously for such a long time
                .event("Transfer(address,address,uint256)") 
                .address(address.parse::<Address>().unwrap()); // The address we monitor
    
        let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?;
    
        while let Some(log) = stream.next().await {
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
    
    /// @dev Subscribe a certain address's all new txs
    /// @param address The address to subscribe
    pub async fn subscribe_address(&self, address: String) -> Result<()> {
        let client = Provider::<Ws>::connect(self.WSS.clone()).await?;
        let client = Arc::new(client);
    
        let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    
        let mut stream = client.subscribe_blocks().await?;
    
        while let Some(log) = stream.next().await {
            println!("block height: {:?}", log.number);
            let height= *(log.number.unwrap().0.get(0).unwrap());

            let fetcher = fetcher::Fetch::new(self.API_Key.clone());
            let _ = fetcher.fetch_address_all_txs( address.as_str(), height, height).await;
        }
    
        Ok(())
    }
    
    /// @dev Monitor mixing service, record the users who interact with it
    pub async fn monitor_mixing_service(&self) -> Result<()> {

        let mixing_services = tools::get_db_address("mixing_service");
    
        let client = Provider::<Ws>::connect(self.WSS.clone()).await?;
        let client = Arc::new(client);
    
        let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    
        let mut stream = client.subscribe_blocks().await?;
    
        while let Some(log) = stream.next().await {
            println!("block height: {:?}", log.number);
            let height= *(log.number.unwrap().0.get(0).unwrap());
            for address in &mixing_services { // Traverse all mixeing service addresses
                let fetcher = fetcher::Fetch::new(self.API_Key.clone());
                let txs = fetcher.fetch_address_all_txs( (*address).as_str(), height, height).await;
    
                for tx in txs.unwrap() { // If there is a mixing service tx in the new block
                    // Record the user
                    tools::write_addresses_db(tx.from);
                }
            }
        }
    
        Ok(())
    }
    
}