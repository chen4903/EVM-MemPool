use crate::utils::tools;
use crate::listener::fetcher;
use ethers::{
    core::types::BlockNumber,
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::{Ok, Result};
use std::{
    thread,
    time::Duration
};

pub struct Message_Robot {
    API_KEY: String,
    WSS: String,
    sender: String, // Email from
    password: String, // Sender's email server password
    smtp_server: String // Email server smtp code
}

impl Message_Robot {

    /// @param api_key Etherscan API kEY
    /// @param wss WSS URL
    /// @param sender Email from
    /// @param password Sender's email server password
    /// @smtp_server Email server smtp code
    pub fn new(api_key: String, wss: String, sender: String, password: String, smtp_server: String) -> Self {
        Message_Robot {
            API_KEY: api_key,
            WSS: wss,
            sender,
            password,
            smtp_server
        }
    }

    /// @dev Create a robot to monitor the address m, and send email to receiver when the m has action
    /// @param address Who to monitor
    /// @param receiver Which email address to receive
    pub async fn message_robot(
        &self,
        address: String, 
        receiver: String, 
    ) -> Result<()> {
        println!("Robot starts to monitor...");
        let client =
        Provider::<Ws>::connect(self.WSS.clone()).await?;
    
        let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    
        let mut stream = client.subscribe_blocks().await?;
    
        while let Some(log) = stream.next().await {
            println!("block height: {:?}", log.number);
            let height= *(log.number.unwrap().0.get(0).unwrap());

            let fetcher = fetcher::Fetch::new(self.API_KEY.clone());
            let txs = fetcher.fetch_address_all_txs( address.as_str(), height, height).await;

            let mut hash = Vec::new();
            for tx in txs.unwrap() {
                hash.push(tx.hash);
            }

            if hash.len() > 0 {
                let content = format!{"Attention! The {} you monitor has action! \nTx hash{:?}", address, hash};

                tools::send_email(self.sender.clone(), receiver.clone(), String::from("SecHelper Robot"), content, self.password.clone(), self.smtp_server.clone()).unwrap();
            }
        }
    
        Ok(())
    }

    /// @dev Create a robot to monitor the address m, and send email to receiver when the m has too many certain tx. 
    /// Check each 30 seconds and the newest 240 blocks. 
    /// @notice 240 blocks ~= 3600 seconds ~= one hour
    /// @param address Who to monitor
    /// @param event The function you call. E.g. `transfer(address,uint256)`
    /// @param receiver Which email address to receive
    /// @param limit The max number of certain txs, rebot will send email as long as the txs number over your limit
    pub async fn warning_robot(&self, address: &str, event: &str, receiver: String, limit: u32) -> Result<()> {
        println!("Robot starts to monitor...");
        let client = Provider::<Ws>::connect(self.WSS.clone()).await?;

        loop {
            let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
            let fetcher = fetcher::Fetch::new(self.API_KEY.clone());

            let txs = fetcher.fetch_address_all_txs(address, last_block.as_u64() - 240, last_block.as_u64()).await;

            let mut count = 0;
            for tx in txs.unwrap() {
                if tx.methodId == tools::function_sig(event) {
                    count = count + 1;
                }
            }

            if count > limit {
                let content = format!{"Warning! The {} you monitor may be in dangerous! \nResult: Too many `{}` txs, which over your limit({})", address, event, limit};

                tools::send_email(self.sender.clone(), receiver.clone(), String::from("SecHelper Robot"), content, self.password.clone(), self.smtp_server.clone()).unwrap();
            }

            thread::sleep(Duration::from_secs(30));
        }

    }
}