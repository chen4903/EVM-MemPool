use crate::utils::tools;
use crate::listener::fetcher;
use ethers::{
    core::types::BlockNumber,
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

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
        let client =
        Provider::<Ws>::connect(self.WSS.clone()).await?;
        let client = Arc::new(client);
    
        let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    
        let mut stream = client.subscribe_blocks().await?;
    
        while let Some(log) = stream.next().await {
            println!("block height: {:?}", log.number);
            let height= *(log.number.unwrap().0.get(0).unwrap());

            let fetcher = fetcher::Fetch::new(self.API_KEY.clone());
            let txs = fetcher.fetch_address_all_txs( address.as_str(), height, height).await;
    
            if txs.unwrap().len() > 0 {
                let content = format!{"Attention! The {} you monitor has action", address};

                tools::send_email(self.sender.clone(), receiver.clone(), String::from("SecHelper Robot"), content, self.password.clone(), self.smtp_server.clone()).unwrap();
            }
        }
    
        Ok(())
    }
}