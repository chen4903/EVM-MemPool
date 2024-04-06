use crate::utils::tools;
use crate::listener::fetcher;
use ethers::{
    core::types::BlockNumber,
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

struct Message_Robot {
    pub api_key: String,
    pub WSS: String,
    pub sender: String, // 哪个邮件作为服务器发邮件
    password: String,
    smtp_server: String
}

impl Message_Robot {
    // 关联函数，用于创建新的Message_Robot实例
    pub fn new(api_key: String, wss: String, sender: String, password: String, smtp_server: String) -> Self {
        Message_Robot {
            api_key,
            WSS: wss,
            sender,
            password,
            smtp_server
        }
    }

    pub async fn message_robot(
        &self,
        address: String, // 你要监听谁
        receiver: String, // 哪个邮件来接收
    ) -> Result<()> {
        let client =
        Provider::<Ws>::connect(self.WSS.clone()).await?;
        let client = Arc::new(client);
    
        let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    
        let mut stream = client.subscribe_blocks().await?;
    
        while let Some(log) = stream.next().await {
            println!("block height: {:?}", log.number);
            let height= *(log.number.unwrap().0.get(0).unwrap());
            let txs = fetcher::fetch_address_all_txs(self.api_key.clone(), address.as_str(), height, height).await;
    
            if txs.unwrap().len() > 0 {
                let content = format!{"Attention! The {} you monitor has action", address};

                tools::send_email(self.sender.clone(), receiver.clone(), String::from("SecHelper Robot"), content, self.password.clone(), self.smtp_server.clone()).unwrap();
            }
        }
    
        Ok(())
    }
}